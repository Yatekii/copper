use schema_parser::geometry::{ Point2D, Vector3, Matrix4, AABB };


pub struct ViewState {
    pub current_perspective: Matrix4,
    pub width: isize,
    pub height: isize,
    pub scale: f32,
    pub center: Point2D,
    pub cursor: Point2D,
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
            current_perspective: Matrix4::identity(),
            width: w as isize,
            height: h as isize,
            scale: 1.0 / 6000.0,
            center: Point2D::origin(),
            cursor: Point2D::origin(),
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

    pub fn update_from_box_pan(&mut self, rect: AABB) {
        let m = (rect.maxs().x - rect.mins().x).max(rect.maxs().y - rect.mins().y);
        if m > 0.0 {
            self.scale = 2.45 / m;
            println!("---------------");
            println!("{:?}", (rect.mins() + rect.maxs().coords));
            println!("{:?}", (rect.mins() + rect.maxs().coords) / 2.0);
            self.center = (rect.mins() + rect.maxs().coords) / 2.0;
            self.update_perspective();
        }
    }

    pub fn update_perspective(&mut self) {
        let aspect_ratio = (self.height as f32) / (self.width as f32);

        self.current_perspective = Matrix4::new_nonuniform_scaling(&Vector3::new(self.scale * aspect_ratio, self.scale, 1.0))
            .prepend_translation(&Vector3::new(self.center.x, self.center.y, 0.0));
    }
}