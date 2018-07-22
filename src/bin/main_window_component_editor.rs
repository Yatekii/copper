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
};

use gdk;
use gdk::{
    EventMask,
    ModifierType,
    EventMotion,
    EventKey,
    EventButton
};

use relm::Widget;
use relm_attributes::widget;

use self::Msg::*;
use components::cursor_info;
use components::info_bar;
use components::cursor_info::CursorInfo;
use components::info_bar::InfoBar;

use copper::state::component_libraries::*;
use copper::state::event::{EventBus, Listener, EventMessage};
use copper::geometry::Point2D;

use copper::viewing::component_viewer;
use copper::drawing::component_drawer;

use copper::loading::component_libraries_loader;
use copper::parsing::component::Component;
use copper::state::schema::ViewState;

pub struct Model {
    current_component: Arc<RwLock<Component>>,
    view_state: Arc<RwLock<ViewState>>,
    component_libraries: Arc<RwLock<ComponentLibraries>>,
    event_bus: EventBus,
    title: String,
    frame_start: Instant,
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
    KeyDown(EventKey)
}

#[widget]
impl Widget for Win {
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

        let context = self.gl_area.get_context().unwrap().clone();
        self.make_context_current(context);
    }
    
    // The initial model.
    fn model() -> Model {
        let event_bus = EventBus::new();

        let view_state = Arc::new(RwLock::new(ViewState::new(1, 1)));
        let libraries = Arc::new(RwLock::new(ComponentLibraries::new(event_bus.get_handle())));

        let args: Vec<String> = env::args().collect();
        if args.len() != 3 {
            println!("Please specify a .lib and a .sch file.");
            ::std::process::exit(1);
        }

        let mut libraries_loader = component_libraries_loader::ComponentLibrariesLoader::new(libraries.clone());
        libraries_loader.load_from_file(&args[1]);

        let component = Arc::new(RwLock::new(libraries.read().unwrap().get_component_by_name("7508110151").unwrap().clone()));

        let drawer: Arc<RwLock<Listener>> = Arc::new(RwLock::new(component_drawer::ComponentDrawer::new(view_state.clone())));
        let viewer: Arc<RwLock<Listener>> = Arc::new(RwLock::new(component_viewer::ComponentViewer::new(component.clone(), view_state.clone(), libraries.clone())));
        event_bus.get_handle().add_listener(drawer);
        event_bus.get_handle().add_listener(viewer);

        view_state.write().unwrap().update_from_box_pan(component.read().unwrap().get_boundingbox().clone());

        Model {
            current_component: component,
            view_state,
            component_libraries: libraries,
            event_bus,
            title: "Component Renderer".to_string(),
            frame_start: Instant::now(),
        }
    }

    // Update the model according to the message received.
    fn update(&mut self, event: Msg) {
        //println!("{:?}", event);
        match event {
            Quit => gtk::main_quit(),
            Realize => println!("realize!"), // This will never be called because relm applies this handler after the event
            Unrealize => println!("unrealize!"),
            RenderGl(context) => {
                self.model.frame_start = Instant::now();
                self.make_context_current(context);
                self.model.event_bus.get_handle().send(&EventMessage::DrawComponent);
                let d = Instant::now() - self.model.frame_start;
                self.info_bar.emit(info_bar::Msg::FrameTimeCaptured(d.as_secs() * 1e6 as u64 + d.subsec_micros() as u64));
                //println!("Frame Duration {}", d.as_secs() * 1e6 as u64 + d.subsec_micros() as u64);
            },
            Resize(w,h, factor) => {
                println!("RenderArea size - w: {}, h: {}", w, h);
                {
                    let mut view_state = self.model.view_state.write().unwrap();
                    view_state.update_from_resize(w as u32, h as u32);
                    self.model.title = format!("Component Renderer {:?}", Point2D::new(w as f32, h as f32));

                    view_state.update_display_scale_factor(factor);

                    self.model.event_bus.get_handle().send(&EventMessage::ResizeDrawArea(w as u16, h as u16));
                }
                self.notify_view_state_changed();
            },
            ButtonPressed(event) => {
                println!("BTN DOWN {:?}", event.get_button());
                if event.get_button() == 1 {
                    let mut view_state = self.model.view_state.write().unwrap();
                    view_state.select_hovered_component();
                }
                self.notify_view_state_changed();
            },
            MoveCursor(event) => {
                {
                    let mut view_state = self.model.view_state.write().unwrap();
                    let (x, y) = event.get_position();
                    let new_state = Point2D::new(x as f32, y as f32);
                    if event.get_state().contains(ModifierType::BUTTON3_MASK) {
                        let mut movement = new_state - view_state.get_cursor();
                        movement.x /= view_state.width as f32 * view_state.get_aspect_ratio();
                        movement.y /= - view_state.height as f32;
                        view_state.center -= movement / view_state.scale * 8.0;
                        view_state.update_perspective();
                    }
                    view_state.update_cursor(new_state);
                }
                self.notify_view_state_changed();
            },
            ZoomOnSchema(_x, y) => {
                {
                    let mut view_state = self.model.view_state.write().unwrap();
                    view_state.update_from_zoom(y as f32);
                }
                self.notify_view_state_changed();
            },
            KeyDown(event) => {
                #[allow(non_upper_case_globals)]
                use gdk::enums::key::{ r };
                let view_state = self.model.view_state.read().unwrap();
                match event.get_keyval() {
                    r => {
                        // view_state.hovered_component_uuid.as_ref().map(|uuid| schema.rotate_component(uuid.clone()));
                    },
                    _ => ()
                }
            }
        }
    }

    fn notify_view_state_changed(&mut self) {
        self.gl_area.queue_draw();
        self.model.event_bus.get_handle().send(&EventMessage::ViewStateChanged);
        let view_state = self.model.view_state.read().unwrap();
        self.cursor_info.emit(cursor_info::Msg::ViewStateChanged(view_state.clone()));
    }

    fn make_context_current(&mut self, context: gdk::GLContext) {
        // Make the GlContext received from GTK the current one
        use gdk::GLContextExt;
        context.make_current();
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

            #[name="main_box"]
            gtk::Box {
                orientation: Vertical,
                can_focus: false,
                spacing: 6,
                realize => Realize,

                #[name="info_bar"]
                InfoBar {

                },

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
                #[name="cursor_info"]
                CursorInfo {

                },
            },
            key_press_event(_, event) => (KeyDown(event.clone()), Inhibit(false)),
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}