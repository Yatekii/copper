use drawing;


pub struct ShapeDrawable {
    buffers: drawing::Buffers
}

impl ShapeDrawable {
    pub fn new(buffers: drawing::Buffers, _color: drawing::Color) -> Self {
        ShapeDrawable {
            buffers
        }
    }
}

impl super::Drawable for ShapeDrawable {
    fn draw(&self, buffers: &mut drawing::Buffers){
        self.buffers.apply_to(buffers);
    }
}