mod circle;
mod line;
mod pin;
mod polygon;
mod rectangle;
mod text;


pub use self::circle::load_circle;
pub use self::line::load_line;
pub use self::pin::load_pin;
pub use self::polygon::load_polygon;
pub use self::rectangle::load_rectangle;
pub use self::text::load_text;


pub const VS_CODE: &[u8] = include_bytes!("../../shaders/shape.glslv");
pub const FS_CODE: &[u8] = include_bytes!("../../shaders/shape.glslf");

pub const VS_RENDER_CODE: &[u8] = include_bytes!("../../shaders/render.glslv");
pub const FS_RENDER_CODE: &[u8] = include_bytes!("../../shaders/render.glslf");