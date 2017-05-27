
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

use std::thread;
use std::sync::{Mutex, Arc};

//Setter
extern crate first_rs_engine;
extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate glutin;
extern crate gfx_device_gl;
extern crate gfx_window_glutin;

use gfx::*;
use first_rs_engine::*;
use cgmath::*;


pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;




pub fn main() {

    let settings = e_engine_settings::EngineSettings::new().with_name("Test Window").with_light_counts(1, 1, 6)
    .with_dimensions(1280, 720);

    //Init window with settings
    let events_loop = glutin::EventsLoop::new();
    let builder = glutin::WindowBuilder::new()
        .with_title(settings.name.clone())
        .with_dimensions(settings.width.clone(), settings.height.clone())
        .with_vsync();
    let (window, mut device, mut factory, mut main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, &events_loop);

    let _ = window.set_cursor_state(glutin::CursorState::Hide);
    //Create encoder
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    //absolute handler
    //Renderer
    let mut renderer: e_renderer::Renderer = e_renderer::Renderer::new();
    //Init all managers
    let mut input_handler: e_input::InputSystem = e_input::InputSystem::new();
    let mut time_handler: e_time::Time = e_time::Time::new();
    //Create first scene (relative handlers are created in it)

    let mut scene_manager: e_scene_mananger::SceneManger<gfx_device_gl::Resources> = e_scene_mananger::SceneManger::new(&settings);

    scene_manager.add_by_name("MyScene", &settings);
    scene_manager.add_by_name("Wall_Scene", &settings);

    //Get reference to creted scene
    {
        scene_manager.add_by_name("LoadingScreen", &settings);
        let loading_screen = scene_manager.get_scene("LoadingScreen");

        //Setup_LoadingScreen
        loading_screen.material_manager.add("Text_mat", "data/Textures/Loading_Screen.png",
                                            "data/Textures/Loading_Screen.png",
                                            "data/Textures/Loading_Screen.png",
                                            1.0, 64.0, 0.8, 1.0);
        //loading_screen.light_manager.add_directional_light("Sun_Loading", e_light::Light_Directional::new(Vector3::new(0.0, 1.0, 0.0),
        //                                    Vector3::new(1.0, 0.95, 0.95), 3.0));

        loading_screen.model_manager.import_model_assimp("Mat", "data/Loading_Screen_Text.fbx", &mut factory,
                                    &mut main_color, &mut main_depth,
                                    &mut loading_screen.material_manager.get_material("Text_mat"),
                                    g_object::MaterialType::OPAQUE,
                                    &loading_screen.light_manager);

        loading_screen.camera.set_frustum_planes(0.1, 500.0);

        //Render_Loading_Screen
        renderer.render(&mut encoder, &loading_screen.camera,
            loading_screen.camera.get_perspective(&settings),
            &mut loading_screen.light_manager,
            &mut loading_screen.model_manager);
        //Send to gpu
        encoder.flush(&mut device);
        //Swap
        window.swap_buffers().unwrap();
        device.cleanup();
        println!("Printing Screen", );
    }

    {
        let mut wall_scene = scene_manager.get_scene("Wall_Scene");
        wall_scene.light_manager.add_directional_light("Sun_Loading", e_light::Light_Directional::new(Vector3::new(-1.0, -0.2, 1.0),
                                            Vector3::new(1.0, 0.95, 0.95), 3.0));
        wall_scene.material_manager.add("Wall_mat",
                            "/share/Photogrammetry/_FinalModels/Seasons/2017_04_Hood/Wall/Wall.png",
                            "/share/Photogrammetry/_FinalModels/Seasons/2017_04_Hood/Wall/Wall_s.png",
                            "/share/Photogrammetry/_FinalModels/Seasons/2017_04_Hood/Wall/Wall_n.png",
                            0.8, 32.0, 0.1, 1.0);
        wall_scene.model_manager.import_model_assimp("Wall", "/share/Photogrammetry/_FinalModels/Seasons/2017_04_Hood/Wall/Wall.fbx", &mut factory,
                                    &mut main_color, &mut main_depth,
                                    &mut wall_scene.material_manager.get_material("Wall_mat"),
                                    g_object::MaterialType::OPAQUE,
                                    &wall_scene.light_manager);
        wall_scene.camera.set_frustum_planes(0.1, 500.0);
        println!("Finished Wall", );
    }

    {
        //Load first level
        let my_scene = scene_manager.get_scene("MyScene");
        //Set camera options
        my_scene.camera.set_frustum_planes(0.1, 500.0);

        //add a default material with some different textures
        my_scene.material_manager.add("standart_material",
                            "data/Textures/fallback_diff.png",
                            "data/Textures/fallback_spec.png",
                            "data/Textures/fallback_nrm.png",
                            0.1, 64.0, 0.8, 1.0);

        my_scene.material_manager.add("cube",
                            "data/Textures/Metal_diff.png",
                            "data/Textures/Metal_diff.png",
                            "data/Cube_Nrm.png",
                            0.1, 64.0, 0.8, 1.0);

        //Add some lights
        my_scene.light_manager.add_directional_light("Sun", e_light::Light_Directional::new(Vector3::new(1.0, -0.5, 1.0),
                                            Vector3::new(1.0, 0.95, 0.95), 3.0));

        my_scene.light_manager.add_point_light("Point", e_light::Light_Point::new(Vector3::new(2.0, -2.0, 2.0),
                                      Vector3::new(1.0, 0.0, 0.0), 1.0, 0.09, 0.032, 1.0));


        my_scene.model_manager.import_model_assimp("Cube", "data/Cube.obj", &mut factory,
                                    &mut main_color, &mut main_depth,
                                    &mut my_scene.material_manager.get_material("cube"),
                                    g_object::MaterialType::OPAQUE,
                                    &my_scene.light_manager);
        my_scene.model_manager.import_model_assimp("Ground", "data/Ground.obj", &mut factory,
                                    &mut main_color, &mut main_depth,
                                    &mut my_scene.material_manager.get_material("standart_material"),
                                    g_object::MaterialType::OPAQUE,
                                    &my_scene.light_manager);

        my_scene.model_manager.print_scene();
        //scene_manager.set_active("MyScene");
        println!("Finished Main level", );
    }

    scene_manager.set_active("MyScene");

    //Test_Offsceen_adding

    let mut already_in: bool = false;
    //let (tx, rx) = mpsc::channel();
    //Main Loop for the game
    'main: loop {

        //Rendering
        {

            //Update time / physics
            time_handler.update();
            //Breaks main loop if got event from input handler
            if input_handler.process_events(&window, &events_loop) {break 'main};

            //Setting scene
            {
                //Activate_scene
                if input_handler.keys.Arrow_Left && scene_manager.is_in("Wall_Scene"){
                    scene_manager.set_active("Wall_Scene");
                }
                //Activate_Scene
                if input_handler.keys.Arrow_Right && scene_manager.is_in("MyScene"){
                    scene_manager.set_active("MyScene");
                }
            }



            //Calc view for this scene
            {
                let mut current_scene = scene_manager.get_active();
                current_scene.camera.calc_view(&input_handler, &mut time_handler);
            }



            {
                let mut current_scene = scene_manager.get_active();

                renderer.render(&mut encoder, &current_scene.camera,
                    current_scene.camera.get_perspective(&settings),
                    &mut current_scene.light_manager,
                    &mut current_scene.model_manager);
            }
            //Send to gpu
            encoder.flush(&mut device);
            //Swap
            window.swap_buffers().unwrap();
            device.cleanup();
        }

        let delta_time: f32 = time_handler.delta_time();
        println!("FPS: {}", 1.0 / time_handler.delta_time());

    }
}
