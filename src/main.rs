
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

    //Get reference to creted scene
    {
        scene_manager.add_by_name("LoadingScreen", &settings);
        let loading_screen = scene_manager.get_scene("LoadingScreen");

        //Setup_LoadingScreen
        loading_screen.material_manager.add("Text_mat", "data/Textures/fallback_diff.png",
                                            "data/Textures/fallback_spec.png",
                                            "data/Textures/fallback_nrm.png",
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
        //Load first level
        let my_scene = scene_manager.get_scene("MyScene");
        //Set camera options
        my_scene.camera.set_frustum_planes(0.1, 2000.0);
        my_scene.camera.set_direction(Vector3::new(0.0, 0.68, 0.73));
        my_scene.camera.set_position(Vector3::new(0.0, 100.0, 200.0));

        //add a default material with some different textures
        my_scene.material_manager.add("standart_material",
                            "data/Textures/fallback_diff.png",
                            "data/Textures/fallback_spec.png",
                            "data/Textures/fallback_nrm.png",
                            0.2, 64.0, 0.8, 1.0);

        my_scene.material_manager.add("cube",
                            "data/Game/Textures/Bars_BaseColor.tga",
                            "data/Game/Textures/Bars_Roughness.tga",
                            "data/Game/Textures/Bars_Normal.tga",
                            0.2, 64.0, 0.8, 1.0);

        my_scene.material_manager.add("Gras",
                            "data/Game/Textures/Gras_Diff.png",
                            "data/Game/Textures/Gras_Spec.png",
                            "data/Game/Textures/Gras_Nrm.png",
                            0.2, 16.0, 0.2, 1.0);

        my_scene.material_manager.add("Terrain",
                            "data/Game/Textures/Terrain_BaseColor.tga",
                            "data/Game/Textures/Terrain_Roughness.tga",
                            "data/Game/Textures/Terrain_Normal.tga",
                            0.2, 16.0, 0.3, 1.0);

        my_scene.material_manager.add("Rock",
                            "data/Game/Textures/Rock_Diff.png",
                            "data/Game/Textures/Rock_Spec.png",
                            "data/Game/Textures/Rock_Nrm.png",
                            0.2, 32.0, 0.5, 1.0);

        my_scene.material_manager.add("Dirt",
                            "data/Game/Textures/Dirt.png",
                            "data/Game/Textures/Dirt_Spec.png",
                            "data/Game/Textures/Dirt_Nrm.png",
                            0.2, 16.0, 0.3, 1.0);

        my_scene.material_manager.add("Gras_Plade",
                            "data/Game/Textures/Pixel_Gras.png",
                            "data/Game/Textures/Pixel_Gras.png",
                            "data/Game/Textures/Dirt_Nrm.png",
                            0.2, 32.0, 0.1, 1.0);

        my_scene.material_manager.add("Top_01",
                            "data/Game/Textures/Pixel_Gras.png",
                            "data/Game/Textures/Pixel_Gras.png",
                            "data/Game/Textures/Low_Nrm.png",
                            0.2, 32.0, 0.5, 1.0);

        my_scene.material_manager.add("Top_02",
                            "data/Game/Textures/Trees_Top_02.png",
                            "data/Game/Textures/Trees_Top_02_Spec.png",
                            "data/Game/Textures/Low_Nrm.png",
                            0.2, 16.0, 0.4, 1.0);

        my_scene.material_manager.add("Low",
                            "data/Game/Textures/Low_Diff.png",
                            "data/Game/Textures/Trees_Top_02_Spec.png",
                            "data/Game/Textures/Low_Nrm.png",
                            0.2, 8.0, 0.1, 1.0);


        //Add some lights
        my_scene.light_manager.add_directional_light("Sun", e_light::Light_Directional::new(Vector3::new(1.0, -0.5, 1.0),
                                            Vector3::new(1.0, 0.95, 0.95), 3.0));

        my_scene.light_manager.add_point_light("Point", e_light::Light_Point::new(Vector3::new(2.0, -2.0, 2.0),
                                      Vector3::new(0.5, 1.0, 0.5), 1.0, 0.0014, 0.000007, 1.0));


        my_scene.model_manager.import_model_assimp("Cube_L", "data/Game/Bars.fbx", &mut factory,
                                    &mut main_color, &mut main_depth,
                                    &mut my_scene.material_manager.get_material("cube"),
                                    g_object::MaterialType::OPAQUE,
                                    &my_scene.light_manager);
        my_scene.model_manager.get_model("Cube_L_Bars").set_world_location(Vector3::new(-80.0, 0.0, 0.0));

        my_scene.model_manager.import_model_assimp("Cube_R", "data/Game/Bars.fbx", &mut factory,
                                    &mut main_color, &mut main_depth,
                                    &mut my_scene.material_manager.get_material("cube"),
                                    g_object::MaterialType::OPAQUE,
                                    &my_scene.light_manager);

        my_scene.model_manager.get_model("Cube_R_Bars").set_world_location(Vector3::new(80.0, 0.0, 0.0));


        my_scene.model_manager.import_model_assimp("ball", "data/Game/ball.fbx", &mut factory,
                                    &mut main_color, &mut main_depth,
                                    &mut my_scene.material_manager.get_material("standart_material"),
                                    g_object::MaterialType::OPAQUE,
                                    &my_scene.light_manager);
        //PlayArea
        my_scene.model_manager.import_model_assimp("Area", "data/Game/Area.fbx", &mut factory,
                                    &mut main_color, &mut main_depth,
                                    &mut my_scene.material_manager.get_material("Rock"),
                                    g_object::MaterialType::OPAQUE,
                                    &my_scene.light_manager);

        my_scene.model_manager.import_model_assimp("PlayArea_out", "data/Game/Terrain.fbx", &mut factory,
                                    &mut main_color, &mut main_depth,
                                    &mut my_scene.material_manager.get_material("Terrain"),
                                    g_object::MaterialType::OPAQUE,
                                    &my_scene.light_manager);

        my_scene.model_manager.import_model_assimp("Area_Gras", "data/Game/Arena_Gras.fbx", &mut factory,
                                    &mut main_color, &mut main_depth,
                                    &mut my_scene.material_manager.get_material("Gras"),
                                    g_object::MaterialType::OPAQUE,
                                    &my_scene.light_manager);

        my_scene.model_manager.import_model_assimp("Area_Marks", "data/Game/Arena_Marks.fbx", &mut factory,
                                    &mut main_color, &mut main_depth,
                                    &mut my_scene.material_manager.get_material("standart_material"),
                                    g_object::MaterialType::OPAQUE,
                                    &my_scene.light_manager);

        my_scene.model_manager.import_model_assimp("Field", "data/Game/Field.fbx", &mut factory,
                                    &mut main_color, &mut main_depth,
                                    &mut my_scene.material_manager.get_material("Dirt"),
                                    g_object::MaterialType::OPAQUE,
                                    &my_scene.light_manager);

        my_scene.model_manager.import_model_assimp("Gras_Planes", "data/Game/Gras_Area.fbx", &mut factory,
                                    &mut main_color, &mut main_depth,
                                    &mut my_scene.material_manager.get_material("Gras_Plade"),
                                    g_object::MaterialType::OPAQUE,
                                    &my_scene.light_manager);

        my_scene.model_manager.import_model_assimp("Trees_Top_01", "data/Game/Trees_Top_01.fbx", &mut factory,
                                    &mut main_color, &mut main_depth,
                                    &mut my_scene.material_manager.get_material("Top_01"),
                                    g_object::MaterialType::OPAQUE,
                                    &my_scene.light_manager);

        my_scene.model_manager.import_model_assimp("Trees_Top_02", "data/Game/Trees_Top_02.fbx", &mut factory,
                                    &mut main_color, &mut main_depth,
                                    &mut my_scene.material_manager.get_material("Top_02"),
                                    g_object::MaterialType::OPAQUE,
                                    &my_scene.light_manager);

        my_scene.model_manager.import_model_assimp("Trees_Low", "data/Game/Trees_Low.fbx", &mut factory,
                                    &mut main_color, &mut main_depth,
                                    &mut my_scene.material_manager.get_material("Low"),
                                    g_object::MaterialType::OPAQUE,
                                    &my_scene.light_manager);

        //Hideing uneeded stuff
        my_scene.model_manager.get_model("Area_Gras_Arena_Gras").set_active(false);
        my_scene.model_manager.get_model("Area_Area").set_active(false);
        my_scene.model_manager.get_model("Area_Marks_Arena_Marks").set_active(false);
        my_scene.model_manager.get_model("Field_Field").set_active(false);


        my_scene.model_manager.print_scene();
        //scene_manager.set_active("MyScene");
        println!("Finished Main level", );
    }

    scene_manager.set_active("MyScene");

    //loading game instance
    let mut game = g_game::Game::new();
    let side_speed = 10.0;

    let mut first_render: bool = true;
    let mut count = 0;
    //Main Loop for the game
    'main: loop {
        //Rendering
        {
            //Update time / physics
            time_handler.update();
            //Breaks main loop if got event from input handler
            if input_handler.process_events(&window, &events_loop) {break 'main};

            //Calc view for this scene
            {
                let mut current_scene = scene_manager.get_active();

                current_scene.camera.calc_view(&input_handler, &mut time_handler);

                if count < 10 {
                    current_scene.camera.set_direction(Vector3::new(0.0, 0.68, 0.73));
                }
            }

            //Update game
            {
                let current_scene = scene_manager.get_active();

                if count < 10 {
                    current_scene.camera.set_direction(Vector3::new(0.0, 0.68, 0.73));
                    count += 1;
                }
                let light_loc = current_scene.model_manager.get_model("ball_ball").get_world_location();
                current_scene.light_manager.get_point_light("Point").unwrap().set_position(Vector3::new(-light_loc.x, -10.0, -light_loc.z));

                //Test for game env
                let loc_l = current_scene.model_manager.get_model("Cube_L_Bars").get_world_location().clone();
                let loc_r = current_scene.model_manager.get_model("Cube_R_Bars").get_world_location().clone();

                if current_scene.model_manager.is_in_manager("ball_ball"){
                    game.update(current_scene.model_manager.get_model("ball_ball"), time_handler.delta_time().clone(),
                            loc_r,
                            loc_l)
                }

                //Game Input Handling
                //L
                if input_handler.keys.Arrow_Up && current_scene.model_manager.is_in_manager("Cube_L_Bars"){
                    game.move_1(current_scene.model_manager.get_model("Cube_L_Bars"), true, time_handler.delta_time().clone());
                }
                if input_handler.keys.Arrow_Down && current_scene.model_manager.is_in_manager("Cube_L_Bars"){
                    game.move_1(current_scene.model_manager.get_model("Cube_L_Bars"), false, time_handler.delta_time().clone());
                }
                //R
                if input_handler.keys.Arrow_Left && current_scene.model_manager.is_in_manager("Cube_R_Bars"){
                    game.move_2(current_scene.model_manager.get_model("Cube_R_Bars"), true, time_handler.delta_time().clone());
                }
                if input_handler.keys.Arrow_Right && current_scene.model_manager.is_in_manager("Cube_R_Bars"){
                    game.move_2(current_scene.model_manager.get_model("Cube_R_Bars"), false, time_handler.delta_time().clone());
                }

                if input_handler.keys.C{
                    current_scene.model_manager.get_model("Gras_Planes_Gras_Area").set_active(false);
                }
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
    }
}
