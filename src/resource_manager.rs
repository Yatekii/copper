use std;
use std::collections::HashMap;
use std::cmp::PartialEq;
use std::cmp::Eq;
use std::hash::{Hash, Hasher};


use glium;
use glium_text_rusttype;


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
    pub display: &'a glium::Display,
    pub text_system: glium_text_rusttype::TextSystem,
    pub fonts: HashMap<FontKey, glium_text_rusttype::FontTexture>
}

impl<'a> ResourceManager<'a> {
    pub fn new(display: &'a glium::Display) -> Self {
        ResourceManager {
            display: display,
            text_system: glium_text_rusttype::TextSystem::new(display),
            fonts: HashMap::new()
        }
    }

    pub fn load_font(&mut self, font_key: FontKey) {
        // Creating a `FontTexture`, which a regular `Texture` which contains the font.
        // Note that loading the systems fonts is not covered by this library.
        let font = glium_text_rusttype::FontTexture::new(
            self.display,
            std::fs::File::open(&std::path::Path::new(&font_key.path)).unwrap(),
            font_key.size,
            glium_text_rusttype::FontTexture::ascii_character_list()
        ).unwrap();
        self.fonts.insert(font_key, font);
    }

    pub fn get_font(&self, font_key: &FontKey) -> Option<&glium_text_rusttype::FontTexture> {
        self.fonts.get(font_key)
    }
}