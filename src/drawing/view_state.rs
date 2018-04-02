use schema_parser::geometry;


pub struct ViewState {
    pub current_perspective: geometry::TSchemaScreen,
    pub width: isize,
    pub height: isize,
    pub scale: f32,
    pub center: geometry::SchemaPoint2D,
    pub cursor: geometry::ScreenPoint2D,
    pub mouse_state: MouseState
}

pub struct MouseState {
    pub left: bool,
    pub middle: bool,
    pub right: bool
}

impl ViewState {
    pub fn new(w: u32, h: u32) -> ViewState {
        let mut vs = ViewState {
            current_perspective: geometry::TSchemaScreen::identity().into(),
            width: w as isize,
            height: h as isize,
            scale: 1.0 / 12000.0,
            center: geometry::SchemaPoint2D::origin(),
            cursor: geometry::ScreenPoint2D::origin(),
            mouse_state: MouseState {
                left: false,
                middle: false,
                right: false
            }
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

    pub fn update_from_box_pan(&mut self, rect: geometry::SchemaRect) {
        let m = (rect.max_x() - rect.min_x()).max(rect.max_y() - rect.min_y());
        if m > 0.0 {
            self.scale = 2.45 / m;
            self.center = (rect.bottom_left() + rect.top_right().to_vector()) / -2.0;
            self.update_perspective();
            println!("Rect: {}", rect);
        }
    }

    pub fn update_perspective(&mut self) {
        let aspect_ratio = (self.height as f32) / (self.width as f32);

        self.current_perspective = geometry::TSchemaScreen::create_scale(self.scale * aspect_ratio, self.scale, 1.0)
            .pre_translate(self.center.to_3d() - geometry::SchemaPoint3D::origin());
    }
}