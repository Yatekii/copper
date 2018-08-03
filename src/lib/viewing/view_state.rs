use std::f32;

use uuid::Uuid;

use geometry::*;
use utils::geometry::*;

/// A struct which holds all the information about the current view into the current schema.
/// This struct also stores the information about visual tooling such as the cursor location,
/// the currently selected component and so on.
#[derive(Clone)]
pub struct ViewState {
    current_perspective: Matrix4,
    width: usize,
    height: usize,
    scale: f32,
    center: Point2,
    cursor: Point2,
    display_scale_factor: i32,
    pub mouse_state: MouseState,
    hovered_component_uuid: Option<Uuid>,
    pub hovered_component_reference: Option<String>,
    selected_component_uuid: Option<Uuid>,
    pub selected_component_reference: Option<String>,
    grid_size: Point2,
    wire_snap_to_grid: bool,
    component_snap_to_grid: bool,
}

/// Defines flags to describe the mouse state.
bitflags! {
    pub struct MouseState: u32 {
        const NONE = 0b00000000;
        const LEFT = 0b00000100;
        const MIDDLE = 0b00000010;
        const RIGHT = 0b00000001;
    }
}

impl ViewState {
    /// Creates a new `ViewState` with a width and a size.
    /// Everything else is set to non-translating values.
    pub fn new(w: usize, h: usize) -> ViewState {
        let mut vs = ViewState {
            current_perspective: Matrix4::identity(),
            width: w,
            height: h,
            scale: 1.0,
            center: Point2::origin(),
            cursor: Point2::origin(),
            display_scale_factor: 1,
            mouse_state: MouseState::NONE,
            hovered_component_uuid: None,
            hovered_component_reference: None,
            selected_component_uuid: None,
            selected_component_reference: None,
            grid_size: Point2::new(100.0, 100.0),
            wire_snap_to_grid: true,
            component_snap_to_grid: true,
        };
        vs.update_perspective();
        vs
    }

    /// Update the `ViewState` when resizing the canvas.
    /// Updates the internal perspective.
    pub fn update_from_resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.update_perspective();
    }

    /// Update the `ViewState` when zooming the canvas.
    /// Updates the internal perspective.
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

    /// Updates the `ViewState` such that the given `AABB` fits into the canvas.
    /// Guarantees a margin of at least 5% of the width or height (whichever is bigger)
    pub fn update_from_box_pan(&mut self, rect: AABB) {
        let aspect_ratio = (self.height as f32) / (self.width as f32);
        let ratio_w = 2.0 / (rect.maxs().x - rect.mins().x) as f32;
        let ratio_h = aspect_ratio * 2.0 / (rect.maxs().y - rect.mins().y) as f32;

        self.scale = ratio_w.min(ratio_h) * 0.9;
        self.center = rect.center();
        self.update_perspective();
    }

    /// Moves the viewport into the canvas by the `translation` vector.
    pub fn move_viewport(&mut self, translation: Vector2) {
        self.center += &transform_vector_2d(&translation, &Matrix4::new_nonuniform_scaling(
            &Vector3::new(
                -self.scale * self.width as f32 / 2.0 * self.get_aspect_ratio(),
                self.scale * self.height as f32 / 2.0,
                1.0
            )
        ).try_inverse().expect("World transform has no inverse. This is a bug. Please report this event."));
        self.update_perspective();
    }

    /// Updates the world transform.
    /// Should be called in each `update_from_*` function to keep it synced.
    pub fn update_perspective(&mut self) {
        self.current_perspective = Matrix4::new_nonuniform_scaling(
            &Vector3::new(
                self.scale * self.get_aspect_ratio(),
                self.scale,
                1.0
            )
        ).prepend_translation(
            &Vector3::new(
                -self.center.x,
                -self.center.y,
                0.
            )
        );
    }

    /// Returns the current perspective.
    pub fn get_perspective(&self) -> Matrix4 {
        self.current_perspective.clone()
    }

    /// Returns the current canvas dimensions.
    pub fn get_canvas_dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Updates the factor which transforms the cursor coordinates into pixel coordinates.
    /// Those coordinates may vary in systems with HiDPI.
    pub fn update_display_scale_factor(&mut self, factor: i32) {
        self.display_scale_factor = factor;
    }

    /// Returns the current cursor location.
    pub fn get_cursor(&self) -> Point2 {
        self.cursor.clone()
    }

    /// Updates the current cursor location.
    pub fn update_cursor(&mut self, cursor: Point2) {
        self.cursor = cursor;
    }

    /// Returns the canvas aspect ratio.
    pub fn get_aspect_ratio(&self) -> f32 {
        (self.height as f32) / (self.width as f32)
    }

    /// Remembers the currently hovered component.
    pub fn update_hovered_component(&mut self, component_uuid: Option<Uuid>, component_reference: Option<String>) {
        self.hovered_component_uuid = component_uuid;
        self.hovered_component_reference = component_reference;
    }

    pub fn get_hovered_component(&self) -> Option<Uuid> {
        self.hovered_component_uuid.clone()
    }

    /// Selects the currently hovered component.
    pub fn select_hovered_component(&mut self) {
        self.selected_component_uuid = self.hovered_component_uuid;
        self.selected_component_reference = self.hovered_component_reference.clone();
    }

    pub fn get_selected_component(&self) -> Option<Uuid> {
        self.selected_component_uuid.clone()
    }

    /// Selects a component directly without the need of hovering it.
    pub fn select_component(&mut self, component_uuid: Option<Uuid>, component_reference: Option<String>) {
        self.selected_component_uuid = component_uuid;
        self.selected_component_reference = component_reference;
    }

    /// Unselects the currently selected component
    pub fn unselect_component(&mut self) {
        self.selected_component_uuid = None;
        self.selected_component_reference = None;
    }

    /// Returns the current cursor position in schema space.
    pub fn get_cursor_in_schema_space(&self) -> Point2 {
        let cursor = correct_cursor_coordinates(&self.cursor, self.width as f32, self.height as f32, 
            self.display_scale_factor);
            transform_point_2d(
                &cursor,
                // View Matrix always has an inverse or we broke other stuff, so unwrap is ok!
                &(&self.current_perspective).try_inverse().expect("World transform has no inverse. This is a bug. Please report this event.")
            )
    }

    /// Returns the current cursor position but with respect to the grid.
    /// This copies the current cursor, snaps it to the grid and returns it.
    pub fn get_grid_snapped_cursor(&self) -> Point2 {
        Point2::new(
            (self.cursor.x / self.grid_size.x).round() * self.grid_size.x,
            (self.cursor.y / self.grid_size.y).round() * self.grid_size.y
        )
    }

    /// Returns the current cursor position but with respect to the grid.
    /// This copies the current cursor, snaps it to the grid and returns it.
    pub fn get_grid_snapped_cursor_in_schema_space(&self) -> Point2 {
        let ciss = self.get_cursor_in_schema_space();
        Point2::new(
            (ciss.x / self.grid_size.x).round() * self.grid_size.x,
            (ciss.y / self.grid_size.y).round() * self.grid_size.y
        )
    }
}