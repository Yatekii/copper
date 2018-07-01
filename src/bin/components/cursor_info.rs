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

use self::Msg::*;

pub struct Model {
    current_cursor_position: Point2D
}

#[derive(Msg)]
pub enum Msg {
    MoveCursor(i32, i32)
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
        //println!("{:?}", event);
        match event {
            MoveCursor(x, y) => {
                println!("KEK");
                self.model.current_cursor_position = Point2D::new(x as f32, y as f32);
                (||{ self.model.current_cursor_position })();
                //self.cursor_position_screen.set_label(&format!("{:?}", self.model.current_cursor_position));
            },
        }
    }

    view! {
        gtk::Box {
            orientation: Vertical,

            //#[name="cursor_position_screen"]
            gtk::Label {
                // { let model = &__relm_model; model } is a hack to fix the macros
                text: &{
                    let pos = self.model.current_cursor_position;
                    format!("Cursor Position Screen Space: {{{:.0} / {:.0}}}", pos.x, pos.y)
                }
            },
        }
    }
}