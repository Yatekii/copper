use gtk;
use gtk::{
    LabelExt,
    WidgetExt,
    Align,
};

use relm::Widget;
use relm_attributes::widget;

pub struct Model {
    library_name: String,
}

#[derive(Msg)]
pub enum Msg {
}

#[widget]
impl Widget for LibraryListboxEntry {
    /// Prepare the initial list of libraries
    fn init_view(&mut self) {
    }

    /// Create the initial model.
    fn model(_relm: &::relm::Relm<Self>, name: String) -> Model {
        Model {
            library_name: name,
        }
    }

    /// Update the model according to the UI event message received.
    fn update(&mut self, event: Msg) {
        match event {
        }
    }

    pub fn get_name(&self) -> String {
        self.model.library_name.clone()
    }

    view! {
        gtk::Label {
            halign: Align::Start,
            text: &self.model.library_name
        }
    }
}