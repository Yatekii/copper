use std::time::Instant;
use std::sync::{
    Arc,
    RwLock,
};

use env;

use gtk;
use gtk::{
    Inhibit,
    OrientableExt,
    WidgetExt,
    GLAreaExt,
    Orientation::{
        Vertical,
    },
    ContainerExt,
    ListBoxExt,
    ListBoxRowExt,
    GridExt,
};

use gdk;
use gdk::EventKey;
use gdk::enums::key::{ Return };

use relm::{
    Widget,
    ContainerWidget,
    Relm
};
use relm_attributes::widget;

use copper::state::component_libraries::*;
use copper::state::event::{EventBus, Listener, EventMessage};
use copper::state::schema::ViewState;
use copper::viewing::component_viewer;
use copper::drawing::component_drawer;
use copper::loading::component_libraries_loader;
use copper::state::schema::component_instance::ComponentInstance;

use self::Msg::*;
use components::library_listbox_entry::LibraryListboxEntry;



pub struct Model {
    view_state: Arc<RwLock<ViewState>>,
    component_libraries: Arc<RwLock<ComponentLibraries>>,
    event_bus: EventBus,
    frame_start: Instant,
    current_library: Option<String>,
    current_component: Option<String>,
    library_list: Vec<String>,
    component_list: Vec<String>,
    relm: Relm<ComponentSelector>,
}

#[derive(Msg)]
pub enum Msg {
    RenderGl(gdk::GLContext),
    Resize(i32, i32, i32),
    SelectLibrary(Option<i32>),
    SelectComponent(Option<i32>),
    InstantiateComponent(ComponentInstance),
    KeyDown(EventKey),
}

#[widget]
impl Widget for ComponentSelector {
    /// Prepare the initial list of libraries
    fn init_view(&mut self) {
        self.update_libraries();
    }

    /// Create the initial model.
    fn model(relm: &Relm<ComponentSelector>, _: ()) -> Model {
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

        let component = Arc::new(RwLock::new(libraries.read().unwrap().get_component_by_name("AMS1117").unwrap().clone()));

        let drawer: Arc<RwLock<Listener>> = Arc::new(RwLock::new(component_drawer::ComponentDrawer::new(view_state.clone())));
        let viewer: Arc<RwLock<Listener>> = Arc::new(RwLock::new(component_viewer::ComponentViewer::new(component.clone(), view_state.clone())));
        event_bus.get_handle().add_listener(drawer);
        event_bus.get_handle().add_listener(viewer);

        view_state.write().unwrap().update_from_box_pan(component.read().unwrap().get_boundingbox().clone());
        event_bus.get_handle().send(&EventMessage::OpenComponent(component.read().unwrap().clone()));

        Model {
            view_state,
            component_libraries: libraries,
            event_bus,
            frame_start: Instant::now(),
            current_library: None,
            current_component: None,
            library_list: Vec::new(),
            component_list: Vec::new(),
            relm: relm.clone(),
        }
    }

    /// Update the model according to the UI event message received.
    fn update(&mut self, event: Msg) {
        match event {
            RenderGl(context) => {
                self.model.frame_start = Instant::now();
                self.make_context_current(context);
                self.model.event_bus.get_handle().send(&EventMessage::DrawComponent);
            },
            Resize(w,h, factor) => {
                {
                    let mut view_state = self.model.view_state.write().unwrap();
                    view_state.update_from_resize(w as usize, h as usize);
                    view_state.update_display_scale_factor(factor);
                    self.model.event_bus.get_handle().send(&EventMessage::ResizeDrawArea(w as u16, h as u16));
                }
                self.notify_view_state_changed();
            },
            SelectLibrary(i) => self.model.current_library = i.map(|i| self.model.library_list[i as usize].clone()),
            SelectComponent(i) => self.model.current_component = i.map(|i| {
                self.update_component(i);
                self.model.component_list[i as usize].clone()
            }),
            KeyDown(event) => {
                match event.get_keyval() {
                    Return => {
                        self.model.relm.stream().emit(InstantiateComponent(self.instantiate_current_component()))
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    }

    /// Notifies all `Listeners` and the `CursorInfo` of the changed ViewState.
    fn notify_view_state_changed(&mut self) {
        self.gl_area.queue_draw();
        self.model.event_bus.get_handle().send(&EventMessage::ViewStateChanged);
    }

    /// Make given `GLContext` the current one.
    fn make_context_current(&mut self, context: gdk::GLContext) {
        // Make the GlContext received from GTK the current one
        use gdk::GLContextExt;
        context.make_current();
    }

    fn update_components(&mut self) {
        update_components(self);
    }

    fn update_component(&mut self, index: i32) {
        update_component(self, index);
    }

    fn update_libraries(&mut self) {
        let libraries = self.model.component_libraries.read().unwrap().get_libraries();
        self.model.library_list = libraries.clone();
        let num_libs = libraries.len();
        for lib in &libraries {
            let _ = self.libraries_listbox.add_widget::<LibraryListboxEntry>(lib.clone());
        }

        if self.model.current_library.is_none() {
            if num_libs > 0 {
                self.model.current_library = Some(libraries[0].clone());
                let row = self.libraries_listbox.get_row_at_index(0);
                self.libraries_listbox.select_row(row.as_ref());
                self.update_components();
            }
        }
    }

    fn _clear_libraries(&mut self) {
        for child in self.libraries_listbox.get_children() {
            self.libraries_listbox.remove(&child);
        }
    }

    fn _choose_library(&mut self, _library_name: &str) {

    }

    pub fn instantiate_current_component(&self) -> ComponentInstance {
        let libs = self.model.component_libraries.read().unwrap();
        let component = libs.get_component_by_name_and_lib(
            &self.model.current_component.clone().unwrap(),
            &self.model.current_library.clone().unwrap()
        ).clone().unwrap().clone();
        component.instantiate()
    }

    view! {
        gtk::Box {
            name: "component-selector-content",
            orientation: Vertical,
            gtk::Grid {
                row_homogeneous: true,
                column_homogeneous: true,

                #[name="gl_area"]
                gtk::GLArea {
                    can_focus: false,
                    hexpand: true,
                    vexpand: true,
                    cell: {
                        left_attach: 0,
                        top_attach: 0,
                        width: 3,
                        height: 1
                    },
                    resize(area, width, height) => Resize(width, height, area.get_scale_factor()),
                    render(area, context) => ({
                        let rgl = RenderGl(context.clone());
                        area.queue_render();
                        rgl
                    }, Inhibit(true)),
                },

                #[name="components_listbox"]
                gtk::ListBox {
                    cell: {
                        left_attach: 3,
                        top_attach: 0,
                        width: 1,
                        height: 1
                    },
                    row_selected(_, row) => SelectComponent(row.clone().map(|w| w.get_index()))
                },

                #[name="libraries_listbox"]
                gtk::ListBox {
                    cell: {
                        left_attach: 4,
                        top_attach: 0,
                        width: 1,
                        height: 1
                    },
                    row_selected(_, row) => SelectLibrary(row.clone().map(|w| w.get_index()))
                },
            },

            gtk::Entry {

            },
            key_press_event(_, event) => (KeyDown(event.clone()), Inhibit(false)),
        },
    }
}

fn update_components(s: &mut ComponentSelector) {
    if let Some(ref current_library) = s.model.current_library {
        let lib = s.model.component_libraries.read().unwrap();
        let components = lib.get_components_from_lib(&current_library.clone());
        let num_comps = components.len();
        s.model.component_list = components.clone().into_iter().map(|c| c.name.clone()).collect();
        for comp in &components {
            let _ = s.components_listbox.add_widget::<LibraryListboxEntry>(comp.name.clone().into());
        }

        if s.model.current_component.is_none() {
            if num_comps > 0 {
                let row = s.components_listbox.get_row_at_index(0);
                s.components_listbox.select_row(row.as_ref());
            }
        }
    }
}

fn update_component(s: &mut ComponentSelector, index: i32) {
    let libs = s.model.component_libraries.read().unwrap();
    let component = libs.get_component_by_name_and_lib(
        &s.model.component_list[index as usize].clone(),
        &s.model.current_library.clone().unwrap()
    ).clone().unwrap().clone();

    s.model.view_state.write().unwrap().update_from_box_pan(component.get_boundingbox().clone());
    s.model.event_bus.get_handle().send(&EventMessage::OpenComponent(component.clone()));
}