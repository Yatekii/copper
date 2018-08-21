use relm::{
    Relm,
    Widget
};
use relm_attributes::widget;

use gtk::{
    LabelExt,
    OrientableExt,
    Orientation::Horizontal,
    WidgetExt,
    BoxExt,
};

use copper::state::schema::component_instance::ComponentInstance;

use self::Msg::*;

pub struct Model {
    relm: Relm<ComponentInspector>,

    component_instance: Option<ComponentInstance>,
}

#[derive(Msg)]
pub enum Msg {
    UpdateComponentInstance(Option<ComponentInstance>)
}

#[widget]
impl Widget for ComponentInspector {
    // The initial model.
    fn model(relm: &Relm<ComponentInspector>, _: ()) -> Model {
        Model {
            relm: relm.clone(),

            component_instance: None,
        }
    }

    // Update the model according to the message received.
    fn update(&mut self, event: Msg) {
        match event {
            UpdateComponentInstance(component_instance) => self.model.component_instance = component_instance
        }
    }

//  pub uuid: Uuid,
//     pub name: String,
//     pub reference: String,
//     pub position: Point2,
//     pub rotation: Matrix4,
//     #[derivative(Debug="ignore", Clone(clone_with="clone_cached_aabb"))]
//     pub bounding_box: Cell<Option<AABB>>

    view! {
        gtk::Box {
            #[name="libraries_listbox"]
            gtk::ListBox {
                visible: self.model.component_instance.is_some(),
                child: {
                    fill: true,
                    expand: true,
                },
                gtk::Label {
                    text: {
                        let u = if let Some(ci) = &self.model.component_instance { ci.uuid.to_string() } else { "".to_owned() };
                        &format!("UUID: {}", u)
                    }
                }
            },
        }
    }
}