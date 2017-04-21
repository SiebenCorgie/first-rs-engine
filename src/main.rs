

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

const CLEAR_COLOR: [f32; 4] = [0.5, 0.5, 1.0, 1.0];

const PI: f32 = 3.141592653589793238;



pub fn main() {

    //ToBeSubclassed

    //camera General
    let mut cameraPos = Vector3::new(0.0, 0.0, 0.0);
    let mut cameraFront = Vector3::new(0.0, 0.0, -1.0);
    let cameraUp = Vector3::new(0.0, 1.0, 0.0);
    //Camera Rotation
    let mut yaw: f32 = 0.0;
    let mut pitch: f32 = 0.0;


    let mut last_time = Instant::now();
    let mut delta_time = 0 as u32;





    let builder = glutin::WindowBuilder::new()
        .with_title("Triangle example".to_string())
        .with_dimensions(1024, 768)
        .with_vsync();
    let (window, mut device, mut factory, mut main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    window.set_cursor_state(glutin::CursorState::Hide);

    let mut win_pos_x = 0 as i32;
    let mut win_pos_y = 0 as i32;

    let mut win_size_x = 0 as i32;
    let mut win_size_y = 0 as i32;

    let win_pos = window.get_position();
    println!("Win Pos: {:?}", win_pos);

    match win_pos {
        Some((x,y)) => {    win_pos_x = x as i32;
                            win_pos_y = y as i32;},
        _ => {},
    }

    let win_size = window.get_inner_size();
    println!("Win_size: {:?}", win_size);

    match win_size {
        Some((x, y)) => {   win_size_x = x as i32;
                            win_size_y = y as i32;},
        _ => {},
    }

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

/*
    let mut test_obj_1 = g_object::Object::new( &mut factory, &mut main_color, &mut main_depth, "data/ape.obj");
    let mut test_obj_2 = g_object::Object::new( &mut factory, &mut main_color, &mut main_depth, "data/ape2.obj");
    let mut test_obj_3 = g_object::Object::new( &mut factory, &mut main_color, &mut main_depth, "data/sphere.obj");
    let mut test_obj_4 = g_object::Object::new( &mut factory, &mut main_color, &mut main_depth, "data/torus.obj");

    let mut locations: Vec<cgmath::Vector3<f32>> = Vec::new();
    locations.push(Vector3::new(0.0, 0.0, 0.0));
    locations.push(Vector3::new(1.0, 0.0, -2.0));
    locations.push(Vector3::new(-3.0, 3.0, -3.0));
    locations.push(Vector3::new(4.0, 4.0, 4.2));


    test_obj_1.set_world_location(locations[0]);
    test_obj_2.set_world_location(locations[1]);
    test_obj_3.set_world_location(locations[2]);
    test_obj_4.set_world_location(locations[3]);
*/

    let mut input_handler: e_input::InputSystem = e_input::InputSystem::new();
    let mut time_handler: e_time::Time = e_time::Time::new();
    let mut camera: g_camera::Camera = g_camera::Camera::new();
    let mut model_manager: e_model_manager::ModelManager<gfx_device_gl::Resources> = e_model_manager::ModelManager::new();


    //model_manager.import_model("teddy", "data/sphere.obj", &mut factory, &mut main_color, &mut main_depth);
    model_manager.import_model("monument", "/share/Photogrammetry/_FinalModels/Journey/Small_Monuments/Buddha_White/Buddha_OBJ.obj", &mut factory, &mut main_color, &mut main_depth);
    model_manager.print_scene();


    'main: loop {

        //Breaks main loop if got event from input handler
        if input_handler.process_events(&window) {break 'main};

        //Process camera/ updated all camera vectors
        camera.calc_view(&input_handler, &mut time_handler);

        let delta_time: f32 = time_handler.delta_time();

        //Corrected Camera Speed
        let camera_speed = 10.0 * delta_time;

        //Input processing [extra]
        {
            //if C is pressed make it possible to escape the window
            //Otherwise the curser always gets captured
            if input_handler.keys.C == false {
                window.set_cursor_state(glutin::CursorState::Hide);
                let change = window.set_cursor_position((win_pos_x + (win_size_x / 2)), (win_pos_y + (win_size_y / 2)) as i32 );
            }else {
                window.set_cursor_state(glutin::CursorState::Normal);
            }
        }


        //DO Transform
        let proj = cgmath::perspective(cgmath::deg(45.0f32), (1024.0/768.0), 1.0, 50.0).into();

        model_manager.render(&mut encoder, &camera, proj);

        //Send to gpu
        encoder.flush(&mut device);
        //Swap
        window.swap_buffers().unwrap();
        device.cleanup();

    }
}


fn to_radians(degree: f32) -> f32 {
    degree * (std::f64::consts::PI / 180.0) as f32
}
