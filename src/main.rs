

extern crate time;
extern crate image;
extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate tobj;
extern crate glutin;
extern crate gfx_device_gl;

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

const CLEAR_COLOR: [f32; 4] = [0.5, 0.5, 1.0, 1.0];
const PI: f32 = 3.141592653589793238;



pub fn main() {

    let (dim_x, dim_y) = (1280, 720);

    let builder = glutin::WindowBuilder::new()
        .with_title("Triangle example".to_string())
        .with_dimensions(dim_x, dim_y)
        .with_vsync();
    let (window, mut device, mut factory, mut main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    window.set_cursor_state(glutin::CursorState::Hide);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();


    //Init all managers
    let mut input_handler: e_input::InputSystem = e_input::InputSystem::new();
    let mut time_handler: e_time::Time = e_time::Time::new();
    let mut camera: g_camera::Camera = g_camera::Camera::new();
    let mut model_manager: e_model_manager::ModelManager<gfx_device_gl::Resources> = e_model_manager::ModelManager::new();
    let mut material_manager: e_material_manager::MaterialManager = e_material_manager::MaterialManager::new();
    let mut light_manager: e_lights_manager::LightManager = e_lights_manager::LightManager::new(2, 10, 10);

    //add a default material with some different textures
    material_manager.add("standart_material",
                        "data/Textures/fallback_diff.png",
                        "data/Textures/fallback_spec.png",
                        "data/Textures/fallback_nrm.png");

    material_manager.add("ape_mat",
                        "data/ape_tex.png",
                        "data/ape_tex.png",
                        "data/ape_tex_nrm.png");

    //Add some lights
    light_manager.add_directional_light("Sun", e_light::Light_Directional::new(Vector3::new(1.0, -1.0, 1.0),
                                        Vector3::new(1.0, 0.95, 0.95), 1.0));
    light_manager.add_point_light("Point", e_light::Light_Point::new(Vector3::new(10.0, 10.0, 10.0),
                                    Vector3::new(1.0, 0.95, 0.95), 1.0, 0.09, 0.032, 1.0));
    light_manager.add_spot_light("Spot", e_light::Light_Spot::new(Vector3::new(-10.0, 0.0, 0.0),
                                Vector3::new(1.0, -1.0, 1.0), Vector3::new(1.0, 0.95, 0.95), 45.0));

    //Add some models
    model_manager.import_model("sphere", "data/sphere.obj", &mut factory, &mut main_color, &mut main_depth, &mut material_manager.get_material("standart_material"));


    model_manager.print_scene();


    'main: loop {
        //Update time / physics
        time_handler.update();


        //Breaks main loop if got event from input handler
        if input_handler.process_events(&window) {break 'main};

        //Process camera/ updated all camera vectors
        camera.calc_view(&input_handler, &mut time_handler);

        let delta_time: f32 = time_handler.delta_time();

        //Corrected Camera Speed
        let camera_speed = 10.0 * delta_time;

        //Input processing [extra]
        {
            //if M is pressed change shininess
            if input_handler.keys.M == true {
                model_manager.import_model("cube", "data/cube.obj",
                                            &mut factory, &mut main_color, &mut main_depth, &mut material_manager.get_material("standart_material"));
            }
            if input_handler.keys.C{
                model_manager.print_scene();
            }
            if input_handler.keys.Arrow_Down & model_manager.is_in_manager("cube_Cube_Cube.001"){

                let speed = 10.0 * time_handler.delta_time();
                model_manager.get_model("cube_Cube_Cube.001").add_world_location(Vector3::new(0.0, -speed, 0.0));
            }
            if input_handler.keys.Arrow_Up & model_manager.is_in_manager("cube_Cube_Cube.001"){

                let speed = 10.0 * time_handler.delta_time();
                model_manager.get_model("cube_Cube_Cube.001").add_world_location(Vector3::new(0.0, speed, 0.0));
            }
            if input_handler.keys.Arrow_Left & model_manager.is_in_manager("cube_Cube_Cube.001"){

                let speed = 10.0 * time_handler.delta_time();
                model_manager.get_model("cube_Cube_Cube.001").add_world_location(Vector3::new(-speed, 0.0, 0.0));
            }
            if input_handler.keys.Arrow_Right & model_manager.is_in_manager("cube_Cube_Cube.001"){

                let speed = 10.0 * time_handler.delta_time();
                model_manager.get_model("cube_Cube_Cube.001").add_world_location(Vector3::new(speed, 0.0, 0.0));
            }

        }


        //DO Transform
        let proj = cgmath::perspective(cgmath::deg(45.0f32), (dim_x as f32/ dim_y as f32), 1.0, 50.0).into();

        model_manager.render(&mut encoder, &camera, proj);

        //Send to gpu
        encoder.flush(&mut device);
        //Swap
        window.swap_buffers().unwrap();
        device.cleanup();

        println!("FPS: {}", 1.0 /time_handler.delta_time());

    }
}


fn to_radians(degree: f32) -> f32 {
    degree * (std::f64::consts::PI / 180.0) as f32
}
