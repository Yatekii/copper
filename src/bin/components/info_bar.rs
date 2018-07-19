use std::time::Instant;

use relm::{
    Widget
};
use relm_attributes::widget;

use gtk::{
    LabelExt,
    OrientableExt,
    Orientation::Horizontal,
};

use self::Msg::*;

pub struct Model {
    frametime_max: u64,
    frametime_min: u64,
    frametime_avg: u64,
    frametime_measurements: u64,
    last_frame_timestamp: Instant,
    frames_total: u64,
    framerate: u64,
}

#[derive(Msg)]
pub enum Msg {
    FrameTimeCaptured(u64)
}

#[widget]
impl Widget for InfoBar {
    // The initial model.
    fn model() -> Model {
        Model {
            frametime_max: 0,
            frametime_min: 9000000,
            frametime_avg: 0,
            frametime_measurements: 0,
            last_frame_timestamp: Instant::now(),
            frames_total: 0,
            framerate: 0,
        }
    }

    // Update the model according to the message received.
    fn update(&mut self, event: Msg) {
        match event {
            FrameTimeCaptured(us) => {
                if us < 16_000 {
                    self.model.frametime_max = self.model.frametime_max.max(us);
                }
                self.model.frametime_min = self.model.frametime_min.min(us);
                self.model.frametime_avg = (
                    self.model.frametime_avg * self.model.frametime_measurements
                  + us
                ) / (self.model.frametime_measurements + 1);
                let diff = Instant::now() - self.model.last_frame_timestamp;
                self.model.framerate = (
                    self.model.framerate * self.model.frames_total
                  + diff.as_secs() * 1_000_000 + diff.subsec_micros() as u64
                ) / (self.model.frames_total + 1);
                self.model.frametime_measurements += 1;
                self.model.frames_total += 1;
                self.model.last_frame_timestamp = Instant::now();
            },
        }
    }

    view! {
        gtk::Box {
            orientation: Horizontal,

            //#[name="cursor_position_screen"]
            gtk::Label {
                text: &{
                    let v = self.model.frametime_max;
                    format!("FTmax [ms]: {} | ", v as f32 / 1e3)
                }
            },
            gtk::Label {
                text: &{
                    let v = self.model.frametime_min;
                    format!("FTmin [ms]: {} | ", v as f32 / 1e3)
                }
            },
            gtk::Label {
                text: &{
                    let v = self.model.frametime_avg;
                    format!("FTavg [ms]: {} | ", v as f32 / 1e3)
                }
            },
            gtk::Label {
                text: &{
                    let v = self.model.framerate;
                    format!("FPS: {:.0} | ", 1.0 / (v as f32 / 1e6))
                }
            },
        }
    }
}