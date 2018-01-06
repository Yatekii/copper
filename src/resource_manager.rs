use std;
use std::collections::HashMap;
use std::cmp::PartialEq;
use std::cmp::Eq;
use std::hash::{Hash, Hasher};
use std::cell::RefCell;
use std::rc::Rc;
use std::io;
use std::io::Read;


use gfx_device_gl;
use gfx;
use gfx_glyph::{GlyphBrushBuilder, GlyphBrush};


type Resources = gfx_device_gl::Resources;
type Factory = gfx_device_gl::Factory;


#[derive(Debug, Clone)]
pub struct FontKey {
    pub path: String
}

impl FontKey {
    pub fn new<I: Into<String>>(path: I) -> FontKey {
        FontKey {
            path: path.into()
        }
    }
}

impl PartialEq for FontKey {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl Eq for FontKey {
    
}

impl Hash for FontKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

pub struct ResourceManager {
    pub factory: gfx_device_gl::Factory,
    pub target: gfx::handle::RenderTargetView<Resources, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>,
    pub depth_stencil: gfx::handle::DepthStencilView<Resources, (gfx::format::D24_S8, gfx::format::Unorm)>,
    pub encoder: gfx::Encoder<Resources, gfx_device_gl::CommandBuffer>,
    fonts: RefCell<HashMap<FontKey, Rc<RefCell<GlyphBrush<'static, Resources, Factory>>>>>
}

impl<'a> ResourceManager {
    pub fn new(factory: gfx_device_gl::Factory, target: gfx::handle::RenderTargetView<Resources, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>, depth_stencil: gfx::handle::DepthStencilView<Resources, (gfx::format::D24_S8, gfx::format::Unorm)>, encoder: gfx::Encoder<Resources, gfx_device_gl::CommandBuffer>) -> Self {
        ResourceManager {
            factory: factory,
            target: target,
            depth_stencil: depth_stencil,
            encoder: encoder,
            fonts: RefCell::new(HashMap::new())
        }
    }

    pub fn load_font(&self, font_key: FontKey) {
        let ttf = std::fs::File::open(&std::path::Path::new(&font_key.path)).unwrap();

        let mut buffer = Vec::new();
        io::BufReader::new(ttf)
            .read_to_end(&mut buffer)
            .expect(&format!(
                "Can't read font file:\nFile: {}",
                font_key.path
            ));

        let glyph_brush = GlyphBrushBuilder::using_font_bytes(buffer).build(self.factory.clone());

        self.fonts.borrow_mut().insert(font_key, Rc::new(RefCell::new(glyph_brush)));
    }

    pub fn get_font(&self, font_key: FontKey) -> Rc<RefCell<GlyphBrush<'static, Resources, Factory>>> {
        {
            let f = self.fonts.borrow();
            if let Some(font) = f.get(&font_key) {
                return font.clone();
            }
        }
        self.load_font(font_key.clone());
        let f = self.fonts.borrow();
        let font = f.get(&font_key);
        // Unwrap is okay as the font will be loaded for sure
        // If the font cannot be loaded an earlier safe switch will trigger
        font.unwrap().clone()
    }
}