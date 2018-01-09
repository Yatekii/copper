use super::Vertex;


pub struct Buffers {
    pub vbo: Vec<Vertex>,
    pub ibo: Vec<u32>
}

impl Buffers {
    pub fn apply_to(&self, buffers: &mut Buffers) {
        let len = buffers.vbo.len() as u32;
        buffers.vbo.extend(&self.vbo);
        buffers.ibo.extend(&self.ibo.iter().map(|x| x + len).collect::<Vec<_>>());
    }
}