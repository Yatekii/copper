use drawing;
use geometry::Matrix4;


pub struct GroupDrawable {
    drawables: Vec<Box<super::Drawable>>,
    transform: Matrix4,
}

impl GroupDrawable {
    pub fn default() -> Self {
        GroupDrawable {
            drawables: Vec::new(),
            transform: Matrix4::identity(),
        }
    }

    pub fn add<T: 'static + super::Drawable>(&mut self, drawable: T) {
        self.drawables.push(Box::new(drawable));
    }

    pub fn from(drawables: Vec<Box<super::Drawable>>) -> Self {
        GroupDrawable {
            drawables: drawables,
            transform: Matrix4::identity(),
        }
    }
}

impl super::Drawable for GroupDrawable {
    fn draw(&self, buffers: &mut drawing::Buffers, transform: bool) {
        if transform {
            buffers.abo.push(drawing::Attributes {
                transform: self.transform.into()
            });
        }
        for drawable in &self.drawables {
            drawable.draw(buffers, false);
        }
    }
    fn get_transform(&self) -> Matrix4 {
        self.transform
    }
    fn set_transform(&mut self, transform: &Matrix4) {
        self.transform = transform.clone();
    }
    fn set_id(&mut self, id: u32) {
        for drawable in &mut self.drawables {
            drawable.set_id(id);
        }
    }
}