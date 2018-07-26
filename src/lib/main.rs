#![recursion_limit="128"]
#![feature(extern_prelude)]
#![feature(nll)]

#[macro_use]
extern crate nom;
extern crate ncollide2d;
extern crate nalgebra;
extern crate lyon;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate gfx_glyph;
extern crate epoxy;
extern crate gfx_core;
extern crate gfx_gl;
#[macro_use]
extern crate derivative;
#[macro_use]
extern crate bitflags;
extern crate uuid;
extern crate owning_ref;

pub mod parsing;
pub mod drawing;
pub mod geometry;
pub mod utils;
pub mod state;
pub mod viewing;
pub mod loading;