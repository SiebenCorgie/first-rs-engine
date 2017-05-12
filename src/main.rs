
//An Example file how you can use the "first_engine"
//crate for your own project

//The steps are
//1. window creation
//2. handler creation (these are usually the same as shown here)
//3. beginning the main loop
// 3.1 processing input, time and camera(s)
// 3.2 do custom stuff on input
// 3.3 render

//You can add and remove materials and objects at any point


#![allow(non_snake_case)]
//Setter
extern crate first_engine;
extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate glutin;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;

use gfx::*;
use first_engine::*;
use cgmath::*;


pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;




pub fn main() {

    let (dim_x, dim_y) = (1920, 1080);

    let builder = glutin::WindowBuilder::new()
        .with_title("Triangle example".to_string())
        .with_dimensions(dim_x, dim_y)
        .with_vsync();
    let (window, mut device, mut factory, mut main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    let _ = window.set_cursor_state(glutin::CursorState::Hide);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();


    //Init all managers
    let mut input_handler: e_input::InputSystem = e_input::InputSystem::new();
    let mut time_handler: e_time::Time = e_time::Time::new();
    let mut camera: g_camera::Camera = g_camera::Camera::new();
    let mut material_manager: e_material_manager::MaterialManager = e_material_manager::MaterialManager::new();
    let mut light_manager: e_lights_manager::LightManager = e_lights_manager::LightManager::new(1, 1, 6);
    let mut model_manager: e_model_manager::ModelManager<gfx_device_gl::Resources> = e_model_manager::ModelManager::new();


    //Define opengl behavoir
    gfx::preset::blend::ALPHA;
    gfx::preset::depth::LESS_EQUAL_TEST;
    gfx::state::CullFace::Back;


    //add a default material with some different textures
    material_manager.add("standart_material",
                        "data/Textures/fallback_diff.png",
                        "data/Textures/fallback_spec.png",
                        "data/Textures/fallback_nrm.png",
                        0.1, 64.0, 0.8, 1.0);

    material_manager.add("Scan",
                        "data/Textures/fallback_diff.png",
                        "data/Textures/fallback_spec.png",
                        "data/Textures/fallback_nrm.png",
                        0.1, 64.0, 0.8, 1.0);

    material_manager.add("gras_mat",
                        "/share/3DFiles/TextureLibary/Gras/Grasplades/Grass_R_02.png",
                        "/share/3DFiles/TextureLibary/Gras/Grasplades/Grass_R_02.png",
                        "/share/3DFiles/TextureLibary/Gras/Grasplades/WaveGras_01_nrm.png",
                        0.5, 0.15, 1.0, 0.05);

    //Add some lights
    light_manager.add_directional_light("Sun", e_light::Light_Directional::new(Vector3::new(1.0, -0.5, 1.0),
                                        Vector3::new(1.0, 0.95, 0.95), 3.0));
/*
    light_manager.add_point_light("Point", e_light::Light_Point::new(Vector3::new(2.0, -2.0, 2.0),
                                  Vector3::new(1.0, 0.0, 0.0), 1.0, 0.09, 0.032, 1.0));

    light_manager.add_point_light("Point2", e_light::Light_Point::new(Vector3::new(-2.0, -2.0, -2.0),
                                    Vector3::new(0.0, 1.0, 0.0), 1.0, 0.09, 0.032, 1.0));

    light_manager.add_point_light("Point3", e_light::Light_Point::new(Vector3::new(3.0, 3.0, 3.0),
                                Vector3::new(0.0, 0.0, 1.0), 1.0, 0.0014, 0.000007, 1.0));

    light_manager.add_point_light("Point4", e_light::Light_Point::new(Vector3::new(-3.0, -3.0, -3.0),
                                Vector3::new(1.0, 1.0, 0.0), 1.0, 0.09, 0.032, 1.0));

    light_manager.add_point_light("Point5", e_light::Light_Point::new(Vector3::new(4.0, 4.0, 4.0),
                                Vector3::new(0.0, 1.0, 1.0), 1.0, 0.09, 0.032, 1.0));

    light_manager.add_point_light("Point6", e_light::Light_Point::new(Vector3::new(-4.0, -4.0, -4.0),
                                Vector3::new(1.0, 0.0, 1.0), 1.0, 0.0014, 0.000007, 1.0));

*/




    light_manager.add_spot_light("Spot", e_light::Light_Spot::new(Vector3::new(-10.0, 0.0, 0.0),
                                Vector3::new(1.0, -1.0, 1.0), Vector3::new(1.0, 0.95, 0.95), 12.5, 17.5,
                                0.09, 0.032, 1.0));

                                /*
    model_manager.import_model_assimp("sphere", "data/gras.obj", &mut factory,
                                &mut main_color, &mut main_depth,
                                &mut material_manager.get_material("gras_mat"),
                                g_object::MaterialType::MASKED,
                                &light_manager);
                                */
    model_manager.import_model_assimp("cube", "data/cube.obj", &mut factory,
                                &mut main_color, &mut main_depth,
                                &mut material_manager.get_material("standart_material"),
                                g_object::MaterialType::OPAQUE,
                                &light_manager);

    model_manager.import_model_assimp("gras", "data/gras.obj", &mut factory,
                                &mut main_color, &mut main_depth,
                                &mut material_manager.get_material("gras_mat"),
                                g_object::MaterialType::MASKED,
                                &light_manager);

    model_manager.print_scene();


    'main: loop {
        //Update time / physics
        time_handler.update();


        //Breaks main loop if got event from input handler
        if input_handler.process_events(&window) {break 'main};

        //Process camera/ updated all camera vectors
        camera.calc_view(&input_handler, &mut time_handler);

        let delta_time: f32 = time_handler.delta_time();


        //Input processing [extra]
        {
            //if M is pressed change shininess
            if input_handler.keys.M == true {
                if model_manager.get_model("gras_gras").get_active(){
                    model_manager.get_model("gras_gras").set_active(false);
                }else{
                    model_manager.get_model("gras_gras").set_active(true);
                }
            }
            if input_handler.keys.C{
                model_manager.print_scene();
            }
            if input_handler.keys.Arrow_Down & model_manager.is_in_manager("gras_gras"){

                let speed = 10.0 * time_handler.delta_time();
                model_manager.get_model("gras_gras").add_world_location(Vector3::new(0.0, -speed, 0.0));
            }
            if input_handler.keys.Arrow_Up & model_manager.is_in_manager("gras_gras"){

                let speed = 10.0 * time_handler.delta_time();
                model_manager.get_model("gras_gras").add_world_location(Vector3::new(0.0, speed, 0.0));
            }
            if input_handler.keys.Arrow_Left & model_manager.is_in_manager("gras_gras"){

                let speed = 10.0 * time_handler.delta_time();
                model_manager.get_model("gras_gras").add_world_location(Vector3::new(-speed, 0.0, 0.0));
            }
            if input_handler.keys.Arrow_Right & model_manager.is_in_manager("gras_gras"){

                let speed = 10.0 * time_handler.delta_time();
                model_manager.get_model("gras_gras").add_world_location(Vector3::new(speed, 0.0, 0.0));
            }
            if input_handler.keys.Arrow_Down {
                //light_manager.get_point_light("Point3").unwrap().set_position(Vector3::new(0.0, -150.0, 0.0));
            }

        }


        //DO Transform
        let proj = cgmath::perspective(cgmath::deg(45.0f32), (dim_x as f32/ dim_y as f32), 1.0, 50.0).into();

        light_manager.get_spot_light("Spot").unwrap().set_direction(-camera.get_direction());
        light_manager.get_spot_light("Spot").unwrap().set_position(camera.get_position());


        model_manager.render(&mut encoder, &camera, proj, &mut light_manager);

        //Send to gpu
        encoder.flush(&mut device);
        //Swap
        window.swap_buffers().unwrap();
        device.cleanup();

        println!("FPS: {}", 1.0 / time_handler.delta_time());

    }
}
