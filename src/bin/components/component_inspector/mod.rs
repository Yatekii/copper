use std::f32;
use std::f32::consts;

use relm::{
    Relm,
    Widget
};
use relm_attributes::widget;

use gtk::{
    LabelExt,
    OrientableExt,
    Orientation::*,
    WidgetExt,
    BoxExt,
    EntryExt,
    EditableSignals
};

use copper::state::schema::component_instance::ComponentInstance;
use copper::geometry::*;

use self::Msg::*;

pub struct Model {
    relm: Relm<ComponentInspector>,

    component_instance: Option<ComponentInstance>,
}

#[derive(Msg)]
pub enum Msg {
    UpdateComponentInstance(Option<ComponentInstance>),
    ComponentInstanceUpdated(ComponentInstance),
    PosXChanged(i32),
    PosYChanged(i32),
    RotChanged(i32),
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
            UpdateComponentInstance(component_instance) => self.model.component_instance = component_instance,
            PosXChanged(x) => {
                self.model.component_instance.as_mut().map(|ci| ci.position.x = x as f32);
                self.model.component_instance.clone().map(|ci| self.model.relm.stream().emit(ComponentInstanceUpdated(ci)));
            },
            PosYChanged(y) => {
                self.model.component_instance.as_mut().map(|ci| ci.position.y = y as f32);
                self.model.component_instance.clone().map(|ci| self.model.relm.stream().emit(ComponentInstanceUpdated(ci)));
            },
            RotChanged(a) => {
                let mat = Matrix4::from_euler_angles(0.0, 0.0, ((a as f32 * (2.0 * consts::PI) / 360.0) as i32) as f32);
                self.model.component_instance.as_mut().map(|ci| ci.rotation = mat);
                self.model.component_instance.clone().map(|ci| self.model.relm.stream().emit(ComponentInstanceUpdated(ci)));
            }
            _ => ()
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
                },
                gtk::Label {
                    text: {
                        let u = if let Some(ci) = &self.model.component_instance { ci.reference.to_string() } else { "".to_owned() };
                        &format!("Reference: {}", u)
                    }
                },
                gtk::Box {
                    orientation: Horizontal,

                    gtk::Label { text: "X:" },
                    gtk::Entry {
                        changed(entry) => {
                            PosXChanged(entry.get_text().unwrap().parse().unwrap_or(0))
                        },
                        text: &if let Some(ci) = &self.model.component_instance { ci.position.x.to_string() } else { "".to_owned() }
                    },
                    gtk::Label { text: "Y:" },
                    gtk::Entry {
                        changed(entry) => {
                            PosYChanged(entry.get_text().unwrap().parse().unwrap_or(0))
                        },
                        text: &if let Some(ci) = &self.model.component_instance { ci.position.y.to_string() } else { "".to_owned() }
                    },
                },
                gtk::Box {
                    orientation: Horizontal,

                    gtk::Label { text: "Angle: " },
                    gtk::Entry {
                        changed(entry) => {
                            RotChanged(entry.get_text().unwrap().parse().unwrap_or(0))
                        },
                        text: &if let Some(ci) = &self.model.component_instance { f32::acos((ci.rotation.trace() - 1.0) / 2.0 / (2.0 * consts::PI) * 360.0).to_string() } else { "".to_owned() }
                    },
                },
            },
        }
    }
}