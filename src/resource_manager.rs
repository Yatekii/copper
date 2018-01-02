use std;
use std::collections::HashMap;
use std::cmp::PartialEq;
use std::cmp::Eq;
use std::hash::{Hash, Hasher};
use std::cell::RefCell;
use std::rc;


use gfx_device_gl;
use gfx;


type Resources = gfx_device_gl::Resources;


#[derive(Debug, Clone)]
pub struct FontKey {
    pub size: u32,
    pub path: String
}

impl PartialEq for FontKey {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.path == other.path
    }
}

impl Eq for FontKey {
    
}

impl Hash for FontKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.size.hash(state);
        self.path.hash(state);
    }
}

pub struct ResourceManager<'a> {
    pub factory: &'a gfx_device_gl::factory::Factory,
    pub target: &'a gfx::handle::RenderTargetView<Resources, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>,
    // pub text_system: glium_text_rusttype::TextSystem,
    // fonts: RefCell<HashMap<FontKey, rc::Rc<glium_text_rusttype::FontTexture>>>
}

impl<'a> ResourceManager<'a> {
    pub fn new(factory: &'a mut gfx_device_gl::factory::Factory, target: &'a gfx::handle::RenderTargetView<Resources, (gfx::format::R8_G8_B8_A8, gfx::format::Unorm)>) -> Self {
        ResourceManager {
            factory: factory,
            target: target
            // text_system: glium_text_rusttype::TextSystem::new(display),
            // fonts: RefCell::new(HashMap::new())
        }
    }

    // pub fn load_font(&self, font_key: FontKey) {
    //     let font = glium_text_rusttype::FontTexture::new(
    //         self.display,
    //         std::fs::File::open(&std::path::Path::new(&font_key.path)).unwrap(),
    //         font_key.size,
    //         glium_text_rusttype::FontTexture::ascii_character_list()
    //     ).unwrap();
    //     self.fonts.borrow_mut().insert(font_key, rc::Rc::new(font));
    // }

    // pub fn get_font(&self, font_key: FontKey) -> rc::Rc<glium_text_rusttype::FontTexture> {
    //     {
    //         let f = self.fonts.borrow();
    //         if let Some(font) = f.get(&font_key) {
    //             return font.clone();
    //         }
    //     }
    //     self.load_font(font_key.clone());
    //     let f = self.fonts.borrow();
    //     let font = f.get(&font_key);
    //     font.unwrap().clone()
    // }
}