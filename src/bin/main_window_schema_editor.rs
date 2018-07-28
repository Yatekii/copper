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
    Orientation::Vertical,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
    StyleContext,
    CssProvider,
    CssProviderExt,
    OverlayExt,
};

use gdk;
use gdk::{
    EventMask,
    ModifierType,
    EventMotion,
    EventKey,
    EventButton,
    Screen,
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
use copper::state::schema::component_instance::ComponentInstance;

use copper::state::schema::*;
use copper::state::component_libraries::*;
use copper::state::event::{EventBus, Listener, EventMessage};
use copper::geometry::Point2;

use copper::loading::schema_loader;
use copper::viewing::schema_viewer;
use copper::drawing::schema_drawer;

use copper::loading::component_libraries_loader;


use copper::utils::geometry::point_to_vector_2d;

pub struct Model {
    view_state: Arc<RwLock<ViewState>>,
    schema: Arc<RwLock<Schema>>,
    event_bus: EventBus,
    title: String,
    frame_start: Instant,
    component_selector: Component<ComponentSelector>,
    relm: Relm<Win>,

    // Visual tooling state
    in_wire_mode: bool,
    current_wire: Option<Uuid>,
    previous_wire: Option<Uuid>,

}

#[derive(Msg)]
pub enum Msg {
    Quit,
    Realize,
    Unrealize,
    RenderGl(gdk::GLContext),
    Resize(i32, i32, i32),
    ButtonPressed(EventButton),
    MoveCursor(EventMotion),
    ZoomOnSchema(f64, f64),
    KeyDown(EventKey),
    InstantiateComponent(ComponentInstance),
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
        let style = include_bytes!("styles/main.css");
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
        self.model.component_selector.widget().hide();
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

        let drawer: Arc<RwLock<Listener>> = Arc::new(RwLock::new(schema_drawer::SchemaDrawer::new(schema.clone(), view_state.clone(), libraries.clone())));
        let viewer: Arc<RwLock<Listener>> = Arc::new(RwLock::new(schema_viewer::SchemaViewer::new(schema.clone(), view_state.clone(), libraries.clone())));
        event_bus.get_handle().add_listener(drawer);
        event_bus.get_handle().add_listener(viewer);

        // Load schema on boot for now
        Self::load_schema(
            &mut schema_loader::SchemaLoader::new(schema.clone()),
            schema.clone(),
            view_state.clone(),
            libraries,
        );

        Model {
            view_state,
            schema,
            event_bus,
            title: "Schema Renderer".to_string(),
            frame_start: Instant::now(),
            component_selector: create_component::<ComponentSelector>(()),
            relm: relm.clone(),

            in_wire_mode: true,
            current_wire: None,
            previous_wire: None,
        }
    }

    /// Update the model according to the UI event message received.
    fn update(&mut self, event: Msg) {
        //println!("{:?}", event);
        match event {
            Quit => gtk::main_quit(),
            Realize => println!("realize!"), // This will never be called because relm applies this handler after the event
            Unrealize => println!("unrealize!"),
            RenderGl(context) => {
                self.model.frame_start = Instant::now();
                self.make_context_current(context);
                self.model.event_bus.get_handle().send(&EventMessage::DrawSchema);
                let d = Instant::now() - self.model.frame_start;
                self.info_bar.emit(info_bar::Msg::FrameTimeCaptured(d.as_secs() * 1e6 as u64 + d.subsec_micros() as u64));
            },
            Resize(w,h, factor) => {
                {
                    let mut view_state = self.model.view_state.write().unwrap();
                    view_state.update_from_resize(w as usize, h as usize);
                    self.model.title = format!("Schema Renderer {:?}", Point2::new(w as f32, h as f32));

                    view_state.update_display_scale_factor(factor);

                    self.model.event_bus.get_handle().send(&EventMessage::ResizeDrawArea(w as u16, h as u16));
                }
                self.notify_view_state_changed();
            },
            // Executed whenever a mouse button is pressed.
            ButtonPressed(event) => {
                // If the left button was pressed, select the underlying component.
                if event.get_button() == 1 {
                    let (cursor, no_comp_selected) = {
                        let mut view_state = self.model.view_state.write().unwrap();
                        (
                            view_state.get_cursor(),
                            view_state.selected_component_uuid.is_none()
                        )
                    };
                    if self.model.in_wire_mode && no_comp_selected {
                        let mut schema = self.model.schema.write().unwrap();
                        if let Some(cw) = self.model.current_wire {
                            if let Some(pw) = self.model.previous_wire {

                            }
                            schema.end_wire(cw, cursor);
                            self.model.previous_wire = self.model.current_wire.clone();
                            self.model.current_wire = Some(schema.start_wire(cursor));
                        } else {
                            self.model.current_wire = Some(schema.start_wire(cursor));
                        }
                    } else {
                        let mut view_state = self.model.view_state.write().unwrap();
                        if no_comp_selected {
                            view_state.select_hovered_component();
                        } else {
                            view_state.unselect_component();
                        }
                    }
                    self.notify_view_state_changed();
                }
            },
            // Executed any time the mouse is moved.
            MoveCursor(event) => {
                {
                    let mut view_state = self.model.view_state.write().unwrap();

                    // Get the current cursor position.
                    let (x, y) = event.get_position();
                    let new_cursor_position = Point2::new(x as f32, y as f32);

                    // If the right mouse button is pressed:
                    if event.get_state().contains(ModifierType::BUTTON3_MASK) {
                        // Pan the viewport.
                        let mut movement = new_cursor_position - view_state.get_cursor();
                        view_state.move_viewport(movement);
                    }

                    // Update the view state with the current cursor position.
                    view_state.update_cursor(new_cursor_position);

                    // If a component is currently selected, move it.
                    let new_pos = point_to_vector_2d(&view_state.get_grid_snapped_cursor_in_schema_space());
                    view_state.selected_component_uuid.clone().map(|u| self.model.schema.write().unwrap().move_component(u, new_pos));
                }
                self.notify_view_state_changed();
            },
            ZoomOnSchema(x, y) => {
                {
                    let mut view_state = self.model.view_state.write().unwrap();
                    #[cfg(windows)]
                    view_state.update_from_zoom(x as f32);
                    #[cfg(not(windows))]
                    view_state.update_from_zoom(y as f32);
                }
                self.notify_view_state_changed();
            },
            KeyDown(event) => {
                use gdk::enums::key::{ r, a };
                let mut schema = self.model.schema.write().unwrap();
                let view_state = self.model.view_state.read().unwrap();
                match event.get_keyval() {
                    r => {
                        view_state.hovered_component_uuid.as_ref().map(|uuid| schema.rotate_component(uuid.clone()));
                    },
                    a => {
                        self.model.component_selector.widget().show();
                    },
                    _ => ()
                }
            },
            InstantiateComponent(mut comp) => {
                let mut view_state = self.model.view_state.write().unwrap();
                let mut schema = self.model.schema.write().unwrap();
                let pos = view_state.get_cursor_in_schema_space();
                comp.position = pos;
                let uuid = schema.add_component(comp);
                view_state.select_component(Some(uuid), Some("??????".into()));
                self.model.component_selector.widget().hide();
            }
        }
    }

    /// Notifies all `Listeners` and the `CursorInfo` of the changed ViewState.
    fn notify_view_state_changed(&mut self) {
        self.gl_area.queue_draw();
        self.model.event_bus.get_handle().send(&EventMessage::ViewStateChanged);
        let view_state = self.model.view_state.read().unwrap();
        self.cursor_info.emit(cursor_info::Msg::ViewStateChanged(view_state.clone()));
    }

    /// Make given `GLContext` the current one.
    fn make_context_current(&mut self, context: gdk::GLContext) {
        // Make the GlContext received from GTK the current one
        use gdk::GLContextExt;
        context.make_current();
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

                    // The main GLArea where the schema will be rendered onto
                    #[name="gl_area"]
                    gtk::GLArea {
                        can_focus: false,
                        hexpand: true,
                        vexpand: true,
                        realize => Realize,
                        unrealize => Unrealize,
                        resize(area, width, height) => Resize(width, height, area.get_scale_factor()),
                        render(area, context) => ({
                            let rgl = RenderGl(context.clone());
                            area.queue_render();
                            rgl
                        }, Inhibit(true)),
                        button_press_event(_, event) => ({
                            ButtonPressed(event.clone())
                        }, Inhibit(false)),
                        motion_notify_event(_, event) => (MoveCursor(event.clone()), Inhibit(false)),
                        scroll_event(_, event) => (ZoomOnSchema(
                            event.get_delta().0,
                            event.get_delta().1,
                        ), Inhibit(false)),
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