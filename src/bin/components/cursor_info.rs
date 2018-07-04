use relm::{
    Widget
};
use relm_attributes::widget;

use gtk::{
    LabelExt,
    OrientableExt,
    Orientation::Vertical,
};

use copper::geometry::*;
use copper::drawing::view_state;
use copper::utils::geometry::*;

use self::Msg::*;

pub struct Model {
    current_cursor_position_screen: Point2D,
    current_cursor_position_schema: Point2D,
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
        }
    }

    // Update the model according to the message received.
    fn update(&mut self, event: Msg) {
        match event {
            ViewStateChanged(vs) => {
                self.model.current_cursor_position_screen = vs.cursor.clone();
                let cursor = correct_cursor_coordinates(&vs.cursor, vs.width as f32, vs.height as f32);
                self.model.current_cursor_position_schema = transform_point_2d(
                    &cursor,
                    // View Matrix always has an inverse or we broke other stuff, so unwrap is ok!
                    &(&vs.current_perspective).try_inverse().unwrap()
                );
            },
        }
    }

    view! {
        gtk::Box {
            orientation: Vertical,

            //#[name="cursor_position_screen"]
            gtk::Label {
                text: &{
                    let pos = self.model.current_cursor_position_screen;
                    format!("Cursor Position Screen Space: {{{:.0} / {:.0}}}", pos.x, pos.y)
                }
            },
            gtk::Label {
                text: &{
                    let pos = self.model.current_cursor_position_schema;
                    format!("Cursor Position Schema Space: {{{:.0} / {:.0}}}", pos.x, pos.y)
                }
            },
        }
    }
}