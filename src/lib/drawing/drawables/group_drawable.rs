use drawing;


pub struct GroupDrawable {
    drawables: Vec<Box<super::Drawable>>
}

impl GroupDrawable {
    pub fn default() -> Self {
        GroupDrawable {
            drawables: Vec::new()
        }
    }

    pub fn add<T: 'static + super::Drawable>(&mut self, drawable: T) {
        self.drawables.push(Box::new(drawable));
    }
}

impl super::Drawable for GroupDrawable {
    fn draw(&self, buffers: &mut drawing::Buffers) {
        for drawable in &self.drawables {
            drawable.draw(buffers);
        }
    }
}