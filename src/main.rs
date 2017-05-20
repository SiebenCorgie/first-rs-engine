
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
    .with_dimensions(1920, 1080);

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
    scene_manager.add("MyScene", &settings);
    //Get reference to creted scene

    {
    scene_manager.add("LoadingScreen", &settings);
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
    }

    //Load first level
    let my_scene = scene_manager.get_scene("MyScene");
    //Set camera options
    my_scene.camera.set_frustum_planes(0.1, 500.0);
    //Define opengl behavoir
    gfx::preset::blend::ALPHA;
    gfx::preset::depth::LESS_EQUAL_TEST;
    gfx::state::CullFace::Back;


    //add a default material with some different textures
    my_scene.material_manager.add("standart_material",
                        "data/Textures/fallback_diff.png",
                        "data/Textures/fallback_spec.png",
                        "data/Textures/fallback_nrm.png",
                        0.1, 64.0, 0.8, 1.0);

    my_scene.material_manager.add("metal",
                        "data/Textures/Metal_diff.png",
                        "data/Textures/Metal_diff.png",
                        "data/Textures/Metal_nrm.png",
                        0.1, 64.0, 0.8, 1.0);

    my_scene.material_manager.add("cube",
                        "data/Textures/Metal_diff.png",
                        "data/Textures/Metal_diff.png",
                        "data/Cube_Nrm.png",
                        0.1, 64.0, 0.8, 1.0);

    my_scene.material_manager.add("gras_mat",
                        "/share/3DFiles/TextureLibary/Gras/Grasplades/Grass_R_02.png",
                        "/share/3DFiles/TextureLibary/Gras/Grasplades/Grass_R_02.png",
                        "/share/3DFiles/TextureLibary/Gras/Grasplades/WaveGras_01_nrm.png",
                        0.5, 0.15, 1.0, 0.05);

    //Add some lights
    //light_manager.add_directional_light("Sun", e_light::Light_Directional::new(Vector3::new(1.0, -0.5, 1.0),
    //                                    Vector3::new(1.0, 0.95, 0.95), 3.0));

    my_scene.light_manager.add_point_light("Point", e_light::Light_Point::new(Vector3::new(2.0, -2.0, 2.0),
                                  Vector3::new(1.0, 0.0, 0.0), 1.0, 0.09, 0.032, 1.0));

    my_scene.light_manager.add_point_light("Point2", e_light::Light_Point::new(Vector3::new(-2.0, -2.0, -2.0),
                                    Vector3::new(0.0, 1.0, 0.0), 1.0, 0.09, 0.032, 1.0));

    my_scene.light_manager.add_point_light("Point3", e_light::Light_Point::new(Vector3::new(3.0, 3.0, 3.0),
                                Vector3::new(0.0, 0.0, 1.0), 1.0, 0.0014, 0.000007, 1.0));

    my_scene.light_manager.add_point_light("Point4", e_light::Light_Point::new(Vector3::new(-3.0, -3.0, -3.0),
                                Vector3::new(1.0, 1.0, 0.0), 1.0, 0.09, 0.032, 1.0));

    my_scene.light_manager.add_point_light("Point5", e_light::Light_Point::new(Vector3::new(4.0, 4.0, 4.0),
                                Vector3::new(0.0, 1.0, 1.0), 1.0, 0.09, 0.032, 1.0));

    my_scene.light_manager.add_point_light("Point6", e_light::Light_Point::new(Vector3::new(-4.0, -4.0, -4.0),
                                Vector3::new(1.0, 0.0, 1.0), 1.0, 0.0014, 0.000007, 1.0));



    my_scene.light_manager.add_spot_light("Spot", e_light::Light_Spot::new(Vector3::new(-10.0, 0.0, 0.0),
                                Vector3::new(1.0, -1.0, 1.0), Vector3::new(1.0, 0.95, 0.95), 12.5, 17.5,
                                0.09, 0.032, 1.0));


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
    my_scene.model_manager.import_model_assimp("Mat_stand", "data/Mat_Stand.obj", &mut factory,
                                &mut main_color, &mut main_depth,
                                &mut my_scene.material_manager.get_material("metal"),
                                g_object::MaterialType::OPAQUE,
                                &my_scene.light_manager);
    my_scene.model_manager.import_model_assimp("Mat", "data/Mat.obj", &mut factory,
                                &mut main_color, &mut main_depth,
                                &mut my_scene.material_manager.get_material("standart_material"),
                                g_object::MaterialType::OPAQUE,
                                &my_scene.light_manager);
    my_scene.model_manager.import_model_assimp("Monky", "data/Monky.obj", &mut factory,
                                &mut main_color, &mut main_depth,
                                &mut my_scene.material_manager.get_material("standart_material"),
                                g_object::MaterialType::OPAQUE,
                                &my_scene.light_manager);
    my_scene.model_manager.import_model_assimp("Text", "data/Text.obj", &mut factory,
                                &mut main_color, &mut main_depth,
                                &mut my_scene.material_manager.get_material("standart_material"),
                                g_object::MaterialType::OPAQUE,
                                &my_scene.light_manager);
    my_scene.model_manager.import_model_assimp("Torus", "data/Torus.obj", &mut factory,
                                &mut main_color, &mut main_depth,
                                &mut my_scene.material_manager.get_material("standart_material"),
                                g_object::MaterialType::OPAQUE,
                                &my_scene.light_manager);

    my_scene.model_manager.get_model("Monky_Monky").add_world_location(cgmath::Vector3::new(0.0, 100.0, 0.0));
    my_scene.model_manager.get_model("Monky_Monky").set_world_scale(cgmath::Vector3::new(1.0, 1.0, 1.0));
    //model_manager.get_model("Monky_Monky").set_world_rotation(cgmath::Basis3::from_angle_x(cgmath::Rad { s: 45.0 }));


    my_scene.model_manager.print_scene();


    'main: loop {
        //Update time / physics
        time_handler.update();


        //Breaks main loop if got event from input handler
        if input_handler.process_events(&window, &events_loop) {break 'main};

        //Process camera/ updated all camera vectors
        my_scene.camera.calc_view(&input_handler, &mut time_handler);

        let delta_time: f32 = time_handler.delta_time();


        //Input processing [extra]
        {
            //if M is pressed change shininess
            if input_handler.keys.M == true {
                if my_scene.model_manager.get_model("gras_gras").get_active(){
                    my_scene.model_manager.get_model("gras_gras").set_active(false);
                }else{
                    my_scene.model_manager.get_model("gras_gras").set_active(true);
                }
            }
            if input_handler.keys.C{
                my_scene.model_manager.print_scene();
            }
            if input_handler.keys.Arrow_Down & my_scene.model_manager.is_in_manager("gras_gras"){

                let speed = 10.0 * time_handler.delta_time();
                my_scene.model_manager.get_model("gras_gras").add_world_location(Vector3::new(0.0, -speed, 0.0));
            }
            if input_handler.keys.Arrow_Up &my_scene. model_manager.is_in_manager("gras_gras"){

                let speed = 10.0 * time_handler.delta_time();
                my_scene.model_manager.get_model("gras_gras").add_world_location(Vector3::new(0.0, speed, 0.0));
            }
            if input_handler.keys.Arrow_Left & my_scene.model_manager.is_in_manager("gras_gras"){

                let speed = 10.0 * time_handler.delta_time();
                my_scene.model_manager.get_model("gras_gras").add_world_location(Vector3::new(-speed, 0.0, 0.0));
            }
            if input_handler.keys.Arrow_Right & my_scene.model_manager.is_in_manager("gras_gras"){

                let speed = 10.0 * time_handler.delta_time();
                my_scene.model_manager.get_model("gras_gras").add_world_location(Vector3::new(speed, 0.0, 0.0));
            }
            if input_handler.keys.Arrow_Down {
                //light_manager.get_point_light("Point3").unwrap().set_position(Vector3::new(0.0, -150.0, 0.0));
            }

        }


        //DO Transform
        //let proj = cgmath::perspective(cgmath::deg(45.0f32), (dim_x as f32/ dim_y as f32), 0.1, 500.0).into();

        my_scene.light_manager.get_spot_light("Spot").unwrap().set_direction(-my_scene.camera.get_direction());
        my_scene.light_manager.get_spot_light("Spot").unwrap().set_position(my_scene.camera.get_position());


        //Doing rendering in Renderer now
        //model_manager.render(&mut encoder, &camera, proj, &mut light_manager);

        renderer.render(&mut encoder, &my_scene.camera, my_scene.camera.get_perspective(&settings), &mut my_scene.light_manager, &mut my_scene.model_manager);
        //Send to gpu
        encoder.flush(&mut device);
        //Swap
        window.swap_buffers().unwrap();
        device.cleanup();

        //println!("FPS: {}", 1.0 / time_handler.delta_time());

    }
}
