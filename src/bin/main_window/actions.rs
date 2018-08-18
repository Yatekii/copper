use uuid::Uuid;

use gdk::{
    ModifierType,
    EventType,
    EventButton,
    EventKey,
    EventMotion,
};

use gtk::WidgetExt;

use main_window::{
    Win,
    EditMode,
};

use std::time::Instant;
use copper::state::event::EventMessage;
use copper::parsing::kicad::schema::{
    WireType,
    WireSegment,
};
use components::info_bar;
use copper::utils::geometry::*;
use copper::geometry::*;
use copper::state::schema::component_instance::ComponentInstance;


const LEFT_MOUSE_BUTTON: u32 = 1;

impl Win {

    pub fn render_gl(&mut self, context: gdk::GLContext) {
        { self.make_context_current(context); }
        {
            self.model.frame_start = Instant::now();
            self.model.event_bus.get_handle().send(&EventMessage::DrawSchema);
        }
        let d = Instant::now() - self.model.frame_start;
        self.send_to_info_bar(info_bar::Msg::FrameTimeCaptured(d.as_secs() * 1e6 as u64 + d.subsec_micros() as u64));
    }

    pub fn resize_canvases(&mut self, w: i32, h: i32, factor: i32) {
        {
            let mut view_state = self.model.view_state.write().unwrap();
            view_state.update_from_resize(w as usize, h as usize);
            view_state.update_display_scale_factor(factor);
            self.model.title = format!("Schema Renderer {}, {}", w, h);
            self.model.event_bus.get_handle().send(&EventMessage::ResizeDrawArea(w as u16, h as u16));
        }
        self.notify_view_state_changed();
    }

    pub fn button_pressed(&mut self, event: EventButton) {
        // If the left button was pressed:
        if event.get_button() == LEFT_MOUSE_BUTTON {
            {
                let mut view_state = self.model.view_state.write().unwrap();
                let schema = self.model.schema.read().unwrap();
                let mut cursor = view_state.get_cursor_in_schema_space();
                match self.model.edit_mode.clone() {
                    EditMode::Component => {
                        // Grab the currently hovered component(s).
                        for item in view_state.selected_items {
                            let ci = schema.get_component_instance(item);
                        }

                        view_state.selected_items.clear();
                        if !view_state.hovered_items.is_empty() {
                            view_state.add_hovered_item_to_selected_items();
                        }
                    },
                    EditMode::None => {
                        // Select the currently hovered component.
                        view_state.selected_items.clear();
                        if !view_state.hovered_items.is_empty() {
                            view_state.add_hovered_item_to_selected_items();
                        }
                        self.model.edit_mode = EditMode::Component;
                    },
                    _ => {}
                };
            }
            self.notify_view_state_changed();
        }
    }

    pub fn button_released(&mut self, event: EventButton) {
        // If the left button was pressed:
        if event.get_button() == LEFT_MOUSE_BUTTON {
            let mut cursor = {
                let mut view_state = self.model.view_state.write().unwrap();
                view_state.get_cursor_in_schema_space()
            };
            {
                match self.model.edit_mode.clone() {
                    EditMode::Wire(wires, _) => {
                        self.update_preview_wires(&cursor);
                        match event.get_event_type() {
                            EventType::ButtonPress => {
                                if wires.len() > 1 {
                                    self.append_one_preview_wire(&cursor)
                                } else {
                                    self.start_new_preview_wire(&cursor);
                                }
                            }
                            EventType::DoubleButtonPress => self.materialize_preview_wire(),
                            _ => {}
                        }
                    },
                    EditMode::Component => {
                        // Select the currently hovered component.
                        let mut view_state = self.model.view_state.write().unwrap();
                        view_state.selected_items.clear();
                        if !view_state.hovered_items.is_empty() {
                            view_state.add_hovered_item_to_selected_items();
                        }
                    },
                    EditMode::None => {
                        // Select the currently hovered component.
                        let mut view_state = self.model.view_state.write().unwrap();
                        view_state.selected_items.clear();
                        if !view_state.hovered_items.is_empty() {
                            view_state.add_hovered_item_to_selected_items();
                        }
                        self.model.edit_mode = EditMode::Component;
                    },
                };
            }
            self.notify_view_state_changed();
        }
    }

    pub fn move_cursor(&mut self, event: EventMotion) {
        {
            let cursor = {
                let mut view_state = self.model.view_state.write().unwrap();

                // Get the current cursor position.
                let (x, y) = event.get_position();
                let new_cursor_position = Point2::new(x as f32, y as f32);

                // If the right mouse button is pressed:
                if event.get_state().contains(ModifierType::BUTTON3_MASK) {
                    // Pan the viewport.
                    let mut movement = new_cursor_position - view_state.get_cursor();
                    view_state.move_viewport(movement);
                }

                // Update the view state with the current cursor position.
                view_state.update_cursor(new_cursor_position);

                view_state.get_cursor_in_schema_space()
            };

            match &mut self.model.edit_mode {
                EditMode::Wire(_, _) => {
                    self.update_preview_wires(&cursor);
                },
                EditMode::Component => {
                    // If a component is currently selected, move it.
                    let mut view_state = self.model.view_state.read().unwrap();
                    let mut schema = self.model.schema.write().unwrap();
                    let new_pos = point_to_vector_2d(&view_state.get_grid_snapped_cursor_in_schema_space());
                    for u in &view_state.selected_items {
                        schema.move_component(u, new_pos);
                    }
                }

                _ => ()
            };
        }
        self.notify_view_state_changed();
    }

    pub fn zoom_on_schema(&mut self, _x: f64, y: f64) {
        let mut view_state = self.model.view_state.write().unwrap();
        view_state.update_from_zoom(y as f32);
    }

    pub fn key_down(&mut self, event: EventKey) {
        use gdk::enums::key::{r, a, w, Escape};
        let mut schema = self.model.schema.write().unwrap();
        let view_state = self.model.view_state.read().unwrap();
        match event.get_keyval() {
            r => {
                let em = self.model.edit_mode.clone();
                match em {
                    EditMode::Component => { view_state.selected_items.iter().for_each(|uuid| schema.rotate_component(&uuid)); },
                    _ => ()
                };
            },
            a => {
                self.model.edit_mode = EditMode::None;
                self.model.component_selector.widget().show();
            },
            w => {
                if let EditMode::Wire(_, _) = self.model.edit_mode {} else {
                    self.model.edit_mode = EditMode::Wire(vec![], true);
                }
            },
            Escape => {
                if let EditMode::Wire(ref mut wires, _) = self.model.edit_mode {
                    let mut drawer = self.model.drawer.write().unwrap();
                    wires.drain(..).for_each(|wire| drawer.remove_wire(wire));
                }
                self.model.edit_mode = EditMode::None;
            }
            _ => ()
        }
    }

    pub fn instantiate_component(&mut self, mut instance: ComponentInstance) {
        let mut view_state = self.model.view_state.write().unwrap();
        let mut schema = self.model.schema.write().unwrap();
        let pos = view_state.get_cursor_in_schema_space();
        instance.position = pos;
        let uuid = schema.add_component(instance);
        view_state.selected_items.insert(uuid);
        self.model.component_selector.widget().hide();
        self.model.edit_mode = EditMode::Component;
    }

    pub fn grid_changed(&mut self) {
        let mut vs = self.model.view_state.write().unwrap();
        use gtk::EntryExt;
        let x = self.grid_x.get_text().and_then(|t| t.parse().ok());
        let y = self.grid_y.get_text().and_then(|t| t.parse().ok());
        if let (Some(x), Some(y)) = (x, y) {
            vs.set_grid_size(x, y);
        }
    }

    fn update_preview_wires(&mut self, cursor: &Point2) {
        if let EditMode::Wire(wires, lw_is_horizontal) = &mut self.model.edit_mode {
            if wires.len() > 1 {
                let mid = wires.len() - 1;
                let (first, second) = wires[..].split_at_mut(mid);
                let previous_wire = &mut first[first.len() - 1];
                let current_wire = &mut second[0];

                if *lw_is_horizontal {
                    previous_wire.end = cursor.clone();
                    previous_wire.end.x = previous_wire.start.x;
                    current_wire.start = previous_wire.end;
                    current_wire.end = cursor.clone();
                } else {
                    previous_wire.end = cursor.clone();
                    previous_wire.end.y = previous_wire.start.y;
                    current_wire.start = previous_wire.end;
                    current_wire.end = cursor.clone();
                }

                let mut drawer = self.model.drawer.write().unwrap();

                drawer.update_wire(previous_wire.clone());
                drawer.update_wire(current_wire.clone());
            }
        }
    }

    fn append_one_preview_wire(&mut self, cursor: &Point2) {
        if let EditMode::Wire(wires, lw_is_horizontal) = &mut self.model.edit_mode {
            // Create a new wire.
            let ws = WireSegment {
                uuid: Uuid::new_v4(),
                kind: WireType::Wire,
                start: cursor.clone(),
                end: cursor.clone(),
            };

            // Remember the new wire
            wires.push(ws.clone());
            // Add the new wire to the drawer.
            self.model.drawer.write().unwrap().add_wire(ws);

            *lw_is_horizontal = !*lw_is_horizontal;
        }
    }

    fn start_new_preview_wire(&mut self, cursor: &Point2) {
        // Create two new wires.
        let wires = vec![
            WireSegment {
                uuid: Uuid::new_v4(),
                kind: WireType::Wire,
                start: cursor.clone(),
                end: cursor.clone(),
            },
            WireSegment {
                uuid: Uuid::new_v4(),
                kind: WireType::Wire,
                start: cursor.clone(),
                end: cursor.clone(),
            }
        ];

        // Add the new wires to the drawer.
        let mut drawer = self.model.drawer.write().unwrap();
        drawer.add_wire(wires[wires.len() - 1].clone());
        drawer.add_wire(wires[wires.len() - 2].clone());
        self.model.edit_mode = EditMode::Wire(wires, true);
    }

    fn materialize_preview_wire(&mut self) {
        if let EditMode::Wire(wires, _) = &mut self.model.edit_mode {
            // First remove all the previewed wire segments from the drawer.
            // Make sure that we don't aquire the lock for too long.
            {
                let mut drawer = self.model.drawer.write().unwrap();
                wires.iter().for_each(|wire| {
                    drawer.remove_wire(wire.clone());
                });
            }
            // Add the finished wire segments to the schema.
            let mut schema = self.model.schema.write().unwrap();
            wires.drain(..).for_each(|wire| {
                schema.add_wire(wire);
            });
        }
    }
}