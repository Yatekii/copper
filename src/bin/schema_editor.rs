#![feature(use_extern_macros)]
#![feature(extern_prelude)]

extern crate lyon;
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate gfx_glyph;
extern crate glutin;

extern crate gtk;
extern crate gdk;
#[macro_use]
extern crate relm;
extern crate relm_attributes;
#[macro_use]
extern crate relm_derive;

extern crate shared_library;

extern crate ncollide2d;
extern crate nalgebra;

extern crate log;
extern crate env_logger;

extern crate copper;

mod main_window_schema_editor;
mod components;

use std::env;
use std::ptr;
use std::path::Path;
use shared_library::dynamic_library::DynamicLibrary;

use main_window_schema_editor::Win;

fn main() {
    // Load libepoxy
    epoxy::load_with(|s| {
        unsafe {
            #[cfg(not(target_os = "windows"))]
            let path: Option<&Path> = None;

            #[cfg(target_os = "windows")]
            let path = Some(Path::new("C:/msys64/mingw64/bin/libepoxy-0.dll"));

            match DynamicLibrary::open(path).unwrap().symbol(s) {
                Ok(v) => v,
                Err(_e) => { /*println!("{}: {}", s, e);*/ ptr::null() },
            }
        }
    });

    // Run the GTK application
    use relm::Widget;
    Win::run(()).unwrap();

    let _ = env_logger::init();
}
