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

use copper::utils::geometry::*;

use copper::geometry::*;
use copper::drawing::view_state;

use self::Msg::*;

pub struct Model {
    current_cursor_position_screen: Point2D,
    current_cursor_position_schema: Point2D,

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
            current_cursor_position_screen: Point2D::new(0.0, 0.0),
            current_cursor_position_schema: Point2D::new(0.0, 0.0),
        }
    }

    // Update the model according to the message received.
    fn update(&mut self, event: Msg) {
        match event {
            ViewStateChanged(vs) => {
                self.model.current_cursor_position_screen = vs.cursor.clone();
                let mut c = vs.cursor.clone();
                c.x =  (c.x / vs.width as f32 * 2.0) * 2.0 - 1.0;
                c.y = -(c.y / vs.height as f32 * 2.0) * 2.0 + 1.0;
                let transformed_cursor = vector_to_point_2d(
                    &vector_from_4d_to_2d(
                        &((&vs.current_perspective).try_inverse().unwrap()
                      * &point_to_vector_4d(
                            &point_from_2d_to_4d(&c)
                        ))
                    )
                );
                self.model.current_cursor_position_schema = transformed_cursor;
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