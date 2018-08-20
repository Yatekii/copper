use drawing;
use geometry::Matrix4;


pub struct ShapeDrawable {
    buffers: drawing::Buffers
}

impl ShapeDrawable {
    pub fn new(buffers: drawing::Buffers) -> Self {
        ShapeDrawable {
            buffers
        }
    }
}

impl super::Drawable for ShapeDrawable {
    fn draw(&self, buffers: &mut drawing::Buffers, transform: bool){
        if transform {
            buffers.abo.push(drawing::Attributes {
                transform: Matrix4::identity().into()
            });
        }
        self.buffers.apply_to(buffers);
    }
    fn get_transform(&self) -> Matrix4 { Matrix4::identity() }
    fn set_transform(&mut self, _transform: &Matrix4) {}
    fn set_id(&mut self, id: u32) {
        for vertex in &mut self.buffers.vbo {
            vertex.id = id;
        }
    }
}