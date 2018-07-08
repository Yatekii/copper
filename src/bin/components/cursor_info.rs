use relm::{
    Widget
};
use relm_attributes::widget;

use gtk::{
    LabelExt,
    OrientableExt,
    Orientation::Horizontal,
};

use copper::geometry::*;
use copper::drawing::view_state;

use self::Msg::*;

pub struct Model {
    current_cursor_position_screen: Point2D,
    current_cursor_position_schema: Point2D,
    current_hovered_component: String,
    current_selected_component: String,
}

#[derive(Msg)]
pub enum Msg {
    ViewStateChanged(view_state::ViewState)
}

#[widget]
impl Widget for CursorInfo {
    // The initial model.
    fn model() -> Model {
        Model {
            current_cursor_position_screen: Point2D::new(0.0, 0.0),
            current_cursor_position_schema: Point2D::new(0.0, 0.0),
            current_hovered_component: String::new(),
            current_selected_component: String::new(),
        }
    }

    // Update the model according to the message received.
    fn update(&mut self, event: Msg) {
        match event {
            ViewStateChanged(vs) => {
                self.model.current_cursor_position_screen = vs.cursor.clone();
                self.model.current_cursor_position_schema = vs.get_cursor_in_schema_space();
                self.model.current_hovered_component = vs.hovered_component_reference.unwrap_or(String::new());
                self.model.current_selected_component = vs.selected_component_reference.unwrap_or(String::new());
            },
        }
    }

    view! {
        gtk::Box {
            orientation: Horizontal,

            //#[name="cursor_position_screen"]
            gtk::Label {
                text: &{
                    let pos = self.model.current_cursor_position_screen;
                    format!("Screen: {{{:.0} / {:.0}}} | ", pos.x, pos.y)
                }
            },
            gtk::Label {
                text: &{
                    let pos = self.model.current_cursor_position_schema;
                    format!("Schema: {{{:.0} / {:.0}}} | ", pos.x, pos.y)
                }
            },
            gtk::Label {
                text: &{
                    let cc = &self.model.current_hovered_component;
                    format!("Hovered: {} | ", cc)
                }
            },
            gtk::Label {
                text: &{
                    let cc = &self.model.current_selected_component;
                    format!("Selected: {}", cc)
                }
            },
        }
    }
}