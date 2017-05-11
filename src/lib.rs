
//Import all the crates and mods
//Should get some helper functions to build the environmant

extern crate time;
extern crate image;
extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate tobj;
extern crate glutin;
extern crate gfx_device_gl;
extern crate assimp;

use gfx::*;

use std::time::{Instant};
use std::path::Path;


use cgmath::*;


pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

mod g_object;
mod e_input;
mod e_time;
mod g_camera;
mod e_model_manager;
mod t_obj_importer;
mod e_material;
mod e_material_manager;
mod e_light;
mod e_lights_manager;
mod t_assimp_importer;


pub fn create_app(win_x: i32, win_y: i32){
    println!("Teddy");
}
