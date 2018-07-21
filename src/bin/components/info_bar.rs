use std::time::Instant;
use std::collections::VecDeque;

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
    frametime_measurements: VecDeque<u64>,
    last_frame_timestamp: Instant,
    framerate_measurements: VecDeque<u64>,
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
            frametime_measurements: VecDeque::new(),
            last_frame_timestamp: Instant::now(),
            framerate_measurements: VecDeque::new(),
            framerate: 0,
        }
    }

    // Update the model according to the message received.
    fn update(&mut self, event: Msg) {
        match event {
            FrameTimeCaptured(us) => {
                self.model.frametime_measurements.push_back(us);
                if self.model.frametime_measurements.len() > 30 {
                    self.model.frametime_measurements.pop_front();
                }
                self.model.frametime_max = self.model.frametime_max.max(us);
                self.model.frametime_min = self.model.frametime_min.min(us);
                self.model.frametime_avg = self.model.frametime_measurements.iter().sum::<u64>() / 30;

                let now = Instant::now();
                let diff = now - self.model.last_frame_timestamp;
                self.model.framerate_measurements.push_back(
                    diff.as_secs() * 1_000_000 + diff.subsec_micros() as u64
                );
                if self.model.framerate_measurements.len() > 30 {
                    self.model.framerate_measurements.pop_front();
                }
                self.model.framerate = self.model.framerate_measurements.iter().sum::<u64>() / 30;
                self.model.last_frame_timestamp = now
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