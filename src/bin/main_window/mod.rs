mod actions;

use std::time::Instant;
use std::sync::{
    Arc,
    RwLock,
};

use env;

use gtk;
use gtk::{
    ContainerExt,
    Inhibit,
    OrientableExt,
    WidgetExt,
    BoxExt,
    GtkWindowExt,
    GLAreaExt,
    Orientation::*,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
    StyleContext,
    CssProvider,
    CssProviderExt,
    OverlayExt,
    EditableSignals,
    EntryExt,
    GestureLongPress,
    GestureLongPressExt,
    GridExt
};

use gdk;
use gdk::{
    EventMask,
    EventMotion,
    EventKey,
    EventButton,
    Screen,
    WindowExt,
};

use relm::{
    Widget,
    create_component,
    Component,
    Relm,
};
use relm_attributes::widget;

use uuid::Uuid;


use self::Msg::*;
use components::cursor_info;
use components::info_bar;
use components::cursor_info::CursorInfo;
use components::info_bar::InfoBar;
use components::component_selector;
use components::component_selector::ComponentSelector;
use components::component_inspector;
use components::component_inspector::ComponentInspector;
use copper::state::schema::component_instance::ComponentInstance;

use copper::state::schema::*;
use copper::state::component_libraries::*;
use copper::state::event::{ EventBus, EventMessage };

use copper::loading::schema_loader;
use copper::viewing::schema_viewer;
use copper::drawing::schema_drawer;

use copper::loading::component_libraries_loader;
use copper::drawing::schema_drawer::SchemaDrawer;
use copper::parsing::kicad::schema::{
    WireSegment,
};

use copper::geometry::*;

#[derive(Clone, Debug)]
pub enum EditMode {
    Wire(Vec<WireSegment>, bool), // wires, last wire is horizontal
    Component,
    None,
}

pub struct Model {
    pub view_state: Arc<RwLock<ViewState>>,
    pub schema: Arc<RwLock<Schema>>,
    pub drawer: Arc<RwLock<SchemaDrawer>>,
    pub viewer: Arc<RwLock<schema_viewer::SchemaViewer>>,
    pub libraries: Arc<RwLock<ComponentLibraries>>,
    pub event_bus: EventBus,
    pub title: String,
    pub frame_start: Instant,
    pub component_selector: Component<ComponentSelector>,
    pub relm: Relm<Win>,

    pub long_press_gesture: Option<GestureLongPress>,

    // Visual tooling state
    pub edit_mode: EditMode,
    pub selection_rectangle: Option<Uuid>,
    pub grabbed_rectangle: Option<Uuid>,
    pub hovered_rectangle: Option<Uuid>,
    pub span_rectangle: Option<Uuid>,
    pub button_pressed_location: Option<Point2>,
}

#[derive(Msg)]
pub enum Msg {
    Quit,
    Realize,
    Unrealize,
    RenderGl(gdk::GLContext),
    Resize(i32, i32, i32),
    ButtonPressed(EventButton),
    ButtonPressedLong(f64, f64),
    ButtonReleased(EventButton),
    MoveCursor(EventMotion),
    ZoomOnSchema(f64, f64),
    KeyDown(EventKey),
    InstantiateComponent(ComponentInstance),
    GridChanged,
    ComponentInstanceUpdated(ComponentInstance)
}

#[widget]
impl Widget for Win {
    /// Executed on the view init event and enables the proper GTK UI events.
    /// Loads the CSS too.
    fn init_view(&mut self) {
        self.window.add_events(
            EventMask::POINTER_MOTION_MASK.bits() as i32 |
                EventMask::SCROLL_MASK.bits() as i32 |
                EventMask::SMOOTH_SCROLL_MASK.bits() as i32 |
                EventMask::BUTTON_PRESS_MASK.bits() as i32 |
                EventMask::BUTTON_RELEASE_MASK.bits() as i32
        );

        self.gl_area.add_events(
            EventMask::POINTER_MOTION_MASK.bits() as i32 |
                EventMask::SCROLL_MASK.bits() as i32 |
                EventMask::SMOOTH_SCROLL_MASK.bits() as i32 |
                EventMask::BUTTON_PRESS_MASK.bits() as i32 |
                EventMask::BUTTON_RELEASE_MASK.bits() as i32
        );

        // Load CSS
        let screen = Screen::get_default().unwrap();
        let style = include_bytes!("../styles/main.css");
        let provider = CssProvider::new();
        provider.load_from_data(style).unwrap();
        StyleContext::add_provider_for_screen(&screen, &provider, STYLE_PROVIDER_PRIORITY_APPLICATION);

        self.schema_overlay.add_overlay(self.model.component_selector.widget());
        let cs = &self.model.component_selector;
        connect!(
            cs@component_selector::Msg::InstantiateComponent(ref c),
            &self.model.relm,
            InstantiateComponent(c.clone())
        );

        let cs = &self.component_inspector;
        connect!(
            cs@component_inspector::Msg::ComponentInstanceUpdated(ref ci),
            &self.model.relm,
            ComponentInstanceUpdated(ci.clone())
        );

        self.model.component_selector.widget().hide();

        self.window.get_window().unwrap().set_event_compression(false);

        let gesture = GestureLongPress::new(&self.gl_area);
        connect!(gesture, connect_pressed(_, x, y), &self.model.relm, ButtonPressedLong(x, y));
        self.model.long_press_gesture = Some(gesture);
    }

    /// Create the initial model.
    fn model(relm: &Relm<Win>, _: ()) -> Model {
        let event_bus = EventBus::new();

        let view_state = Arc::new(RwLock::new(ViewState::new(1, 1)));
        let schema = Arc::new(RwLock::new(Schema::new(event_bus.get_handle())));
        let libraries = Arc::new(RwLock::new(ComponentLibraries::new(event_bus.get_handle())));

        let args: Vec<String> = env::args().collect();
        if args.len() != 3 {
            println!("Please specify a .lib and a .sch file.");
            ::std::process::exit(1);
        }

        let mut libraries_loader = component_libraries_loader::ComponentLibrariesLoader::new(libraries.clone());
        libraries_loader.load_from_file(&args[1]);

        let drawer: Arc<RwLock<SchemaDrawer>> = Arc::new(RwLock::new(schema_drawer::SchemaDrawer::new(schema.clone(), view_state.clone(), libraries.clone())));
        let viewer: Arc<RwLock<schema_viewer::SchemaViewer>> = Arc::new(RwLock::new(schema_viewer::SchemaViewer::new(schema.clone(), view_state.clone(), libraries.clone())));
        event_bus.get_handle().add_listener(drawer.clone());
        event_bus.get_handle().add_listener(viewer.clone());

        // Load schema on boot for now
        Self::load_schema(
            &mut schema_loader::SchemaLoader::new(schema.clone()),
            schema.clone(),
            view_state.clone(),
            libraries.clone(),
        );

        Model {
            view_state,
            schema,
            drawer,
            viewer,
            libraries,
            event_bus,
            title: "Schema Renderer".to_string(),
            frame_start: Instant::now(),
            component_selector: create_component::<ComponentSelector>(()),
            relm: relm.clone(),
            long_press_gesture: None,

            edit_mode: EditMode::None,
            selection_rectangle: None,
            grabbed_rectangle: None,
            hovered_rectangle: None,
            span_rectangle: None,
            button_pressed_location: None,
        }
    }

    /// Update the model according to the UI event message received.
    fn update(&mut self, event: Msg) {
        //println!("{:?}", event);
        match event {
            Quit => gtk::main_quit(),
            Realize => println!("realize!"), // This will never be called because relm applies this handler after the event
            Unrealize => println!("unrealize!"),
            RenderGl(context) => self.render(context),
            Resize(w, h, factor) => self.resize_canvases(w, h, factor),
            // Executed whenever a mouse button is pressed.
            ButtonPressed(event) => self.button_pressed(event),
            // Executed whenever a mouse button is pressed for a while without moving.
            ButtonPressedLong(x, y) => self.button_pressed_long(x, y),
            // Executed whenever a mouse button is released.
            ButtonReleased(event) => self.button_released(event),
            // Executed any time the mouse is moved.
            MoveCursor(event) => self.move_cursor(event),
            ZoomOnSchema(x, y) => self.zoom_on_schema(x, y),
            KeyDown(event) => self.key_down(event),
            InstantiateComponent(comp) => self.instantiate_component(comp),
            GridChanged => self.grid_changed(),
            ComponentInstanceUpdated(ci) => self.model.event_bus.get_handle().send(&EventMessage::UpdateComponent(ci)),
        }
    }

    fn render(&mut self, context: gdk::GLContext) {
        self.render_gl(context);
        self.gl_area.queue_draw();
    }

    /// Notifies all `Listeners` and the `CursorInfo` of the changed ViewState.
    fn notify_view_state_changed(&mut self) {
        self.model.event_bus.get_handle().send(&EventMessage::ViewStateChanged);
        let view_state = self.model.view_state.read().unwrap();
        self.cursor_info.emit(cursor_info::Msg::ViewStateChanged(view_state.clone()));
        self.gl_area.queue_draw();
    }

    /// Make given `GLContext` the current one.
    fn make_context_current(&mut self, context: gdk::GLContext) {
        // Make the GlContext received from GTK the current one
        use gdk::GLContextExt;
        context.make_current();
    }

    pub fn send_to_info_bar(&mut self, msg: info_bar::Msg) {
        self.info_bar.emit(msg);
    }

    pub fn send_to_component_inspector(&mut self, msg: component_inspector::Msg) {
        self.component_inspector.emit(msg);
    }

    /// Loads a `Schema` from a file given in the `env::args`.
    fn load_schema(schema_loader: &mut schema_loader::SchemaLoader, schema: Arc<RwLock<Schema>>, view_state: Arc<RwLock<ViewState>>, libraries: Arc<RwLock<ComponentLibraries>>) {
        /*
        * L O A D   S C H E M A
        */

        // Load library and schema file
        let args: Vec<String> = env::args().collect();

        // Load a schema form a file specified on the commandline
        schema_loader.load_from_file(args[2].clone());

        // Zoom to BB
        let bb = schema.write().unwrap().get_bounding_box(&libraries.read().unwrap());
        view_state.write().unwrap().update_from_box_pan(bb);
    }

    view! {
        #[name="window"]
        gtk::Window {
            can_focus: false,
            border_width: 1,
            property_default_width: 1800,
            property_default_height: 1000,
            realize => Realize,
            title: &self.model.title,

            child: {
                expand: true,
                fill: true,
            },

            #[name="schema_overlay"]
            gtk::Overlay {
                #[container]
                #[name="main_box"]
                gtk::Box {
                    orientation: Vertical,
                    can_focus: false,
                    spacing: 0,
                    realize => Realize,

                    // An info bar at the top of the window to show metrics like FPS and the likes.
                    #[name="info_bar"]
                    InfoBar {

                    },

                    gtk::Box {
                        orientation: Horizontal,

                        #[name="grid_x"]
                        gtk::Entry {
                            changed(_) => GridChanged,
                            text: &self.model.view_state.read().unwrap().get_grid_size().x.to_string(),
                        },
                        #[name="grid_y"]
                        gtk::Entry {
                            changed(_) => GridChanged,
                            text: &self.model.view_state.read().unwrap().get_grid_size().y.to_string(),
                        },
                    },

                    gtk::Grid {
                        row_homogeneous: true,
                        column_homogeneous: true,
                        // The main GLArea where the schema will be rendered onto
                        #[name="gl_area"]
                        gtk::GLArea {

                            cell: {
                                left_attach: 0,
                                top_attach: 0,
                                width: 4,
                                height: 1
                            },

                            can_focus: false,
                            hexpand: true,
                            vexpand: true,
                            realize => Realize,
                            unrealize => Unrealize,
                            resize(area, width, height) => Resize(width, height, area.get_scale_factor()),
                            render(area, context) => ({
                                let rgl = RenderGl(context.clone());
                                //area.queue_render();
                                rgl
                            }, Inhibit(true)),
                            button_press_event(_, event) => ({
                                ButtonPressed(event.clone())
                            }, Inhibit(false)),
                            button_release_event(_, event) => ({
                                ButtonReleased(event.clone())
                            }, Inhibit(false)),
                            motion_notify_event(_, event) => (MoveCursor(event.clone()), Inhibit(false)),
                            scroll_event(_, event) => (ZoomOnSchema(
                                event.get_delta().0,
                                event.get_delta().1,
                            ), Inhibit(false)),
                        },

                        #[name="component_inspector"]
                        ComponentInspector {
                            cell: {
                                left_attach: 4,
                                top_attach: 0,
                                width: 1,
                                height: 1
                            },
                        },
                    },

                    // An infopane at the bottom of the window to display cursor position, selected component and the likes.
                    #[name="cursor_info"]
                    CursorInfo {

                    },
                },
            },

            key_press_event(_, event) => (KeyDown(event.clone()), Inhibit(false)),
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

