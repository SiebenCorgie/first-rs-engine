
//Engine Crate

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

pub mod g_object;
pub mod e_input;
pub mod e_time;
pub mod g_camera;
pub mod e_model_manager;
pub mod t_obj_importer;
pub mod e_material;
pub mod e_material_manager;
pub mod e_light;
pub mod e_lights_manager;
pub mod t_assimp_importer;
