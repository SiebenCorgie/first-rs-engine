
//Engine Crate

extern crate time;
extern crate image;
extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate gfx_device_gl;
extern crate assimp;
//Sound
extern crate ears;
use ears::{Sound, AudioController};

//gfx
use gfx::*;
//System
use std::time::{Instant};
use std::path::Path;


use cgmath::*;


pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

//ENGINE
//Gameplay
pub mod g_object;
pub mod e_model_manager;
pub mod e_input;

//Misc
pub mod e_engine_settings;


//Renderinga
pub mod e_light;
pub mod e_lights_manager;
pub mod e_material;
pub mod e_material_manager;
pub mod e_scene;
pub mod e_scene_mananger;
//_core
pub mod e_renderer;

//Physic
pub mod e_time;

//TOOLS
pub mod t_assimp_importer;

//GAME
//gameplay
pub mod g_camera;
pub use g_camera::*;
pub mod g_game;
