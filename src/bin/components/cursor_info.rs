use relm::{
    Relm,
    Widget
};
use relm_attributes::widget;

use gtk::{
    LabelExt,
    ButtonExt,
    ContainerExt,
    Inhibit,
    OrientableExt,
    WidgetExt,
    BoxExt,
    GtkWindowExt,
    GLAreaExt,
    Orientation::Vertical,
};

use copper::geometry::*;
use copper::drawing::view_state;

use self::Msg::*;

pub struct Model {
    current_cursor_position: Point2D

}

#[derive(Msg)]
pub enum Msg {
    ViewStateChanged(view_state::ViewState)
}

fn coords_to_string(x: i32, y: i32) {

}

#[widget]
impl Widget for CursorInfo {
    // The initial model.
    fn model() -> Model {
        Model {
            current_cursor_position: Point2D::new(0.0, 0.0)
        }
    }

    // Update the model according to the message received.
    fn update(&mut self, event: Msg) {
        match event {
            ViewStateChanged(vs) => {
                self.model.current_cursor_position = Point2D::new(
                    vs.cursor.x as f32,
                    vs.cursor.y as f32
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
                    let pos = self.model.current_cursor_position;
                    format!("Cursor Position Screen Space: {{{:.0} / {:.0}}}", pos.x, pos.y)
                }
            },
        }
    }
}