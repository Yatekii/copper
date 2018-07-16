use uuid::Uuid;

use geometry::*;
use utils::geometry::*;

#[derive(Clone)]
pub struct ViewState {
    pub current_perspective: Matrix4,
    pub width: isize,
    pub height: isize,
    pub scale: f32,
    pub center: Point2D,
    cursor: Point2D,
    display_scale_factor: i32,
    pub mouse_state: MouseState,
    pub hovered_component_uuid: Option<Uuid>,
    pub hovered_component_reference: Option<String>,
    pub selected_component_uuid: Option<Uuid>,
    pub selected_component_reference: Option<String>,
}


bitflags! {
    pub struct MouseState: u32 {
        const NONE = 0b00000000;
        const LEFT = 0b00000100;
        const MIDDLE = 0b00000010;
        const RIGHT = 0b00000001;
    }
}

impl ViewState {
    pub fn new(w: u32, h: u32) -> ViewState {
        let mut vs = ViewState {
            current_perspective: Matrix4::identity(),
            width: w as isize,
            height: h as isize,
            scale: 1.0 / 6000.0,
            center: Point2D::origin(),
            cursor: Point2D::origin(),
            display_scale_factor: 1,
            mouse_state: MouseState::NONE,
            hovered_component_uuid: None,
            hovered_component_reference: None,
            selected_component_uuid: None,
            selected_component_reference: None,
        };
        vs.update_perspective();
        vs
    }

    pub fn update_from_resize(&mut self, width: u32, height: u32) {
        self.width = width as isize;
        self.height = height as isize;
        self.update_perspective();
    }

    pub fn update_from_zoom(&mut self, delta: f32) {
        self.scale += delta / 10000.0;
        if self.scale < 1.0 / 60000.0 {
            self.scale = 1.0 / 60000.0;
        }
        if self.scale > 0.3 {
            self.scale = 0.3;
        }
        self.update_perspective();
    }

    pub fn update_from_box_pan(&mut self, rect: AABB) {
        let m = (rect.maxs().x - rect.mins().x).max(rect.maxs().y - rect.mins().y);
        if m > 0.0 {
            self.scale = 2.45 / m * 2.0;
            self.center = rect.center();
            self.update_perspective();
        }
    }

    pub fn update_perspective(&mut self) {
        let aspect_ratio = (self.height as f32) / (self.width as f32);
        self.current_perspective = Matrix4::new_nonuniform_scaling(
            &Vector3::new(
                self.scale * aspect_ratio,
                self.scale,
                1.0
            )
        ).prepend_translation(
            &Vector3::new(
                -self.center.x / 2.0,
                -self.center.y / 2.0,
                0.
            )
        );
    }

    pub fn update_display_scale_factor(&mut self, factor: i32) {
        self.display_scale_factor = factor;
    }

    pub fn get_cursor(&self) -> Point2D {
        return self.cursor.clone();
    }

    pub fn update_cursor(&mut self, cursor: Point2D) {
        self.cursor = cursor;
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        (self.height as f32) / (self.width as f32)
    }

    pub fn update_hovered_component(&mut self, component_uuid: Option<Uuid>, reference: Option<String>) {
        self.hovered_component_uuid = component_uuid;
        self.hovered_component_reference = reference;
    }

    pub fn select_hovered_component(&mut self) {
        self.selected_component_uuid = self.hovered_component_uuid;
        self.selected_component_reference = self.hovered_component_reference.clone();
    }

    pub fn get_cursor_in_schema_space(&self) -> Point2D {
        let cursor = correct_cursor_coordinates(&self.cursor, self.width as f32, self.height as f32, 
            self.display_scale_factor);
            transform_point_2d(
                &cursor,
                // View Matrix always has an inverse or we broke other stuff, so unwrap is ok!
                &(&self.current_perspective).try_inverse().unwrap()
            )
    }
}