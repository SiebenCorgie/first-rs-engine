//The render, heart of the engine, currently only sends and get the single forward pass
use g_object;
use e_model_manager;

use g_camera::{CameraTyp, Camera};

use gfx;

use gfx::traits::FactoryExt;
use gfx::{Resources, Bundle, texture, Device};
use cgmath::*;

use e_light;
use e_lights_manager;



//Defs
const CLEAR_COLOR: [f32; 4] = [0.7, 0.91, 0.92, 1.0];


//Render object
pub struct Renderer {}


impl Renderer {

    pub fn new() -> Self{
        Renderer{}
    }

    //Main Rendering Function
    pub fn render<C, R>(&self, encoder: &mut gfx::Encoder<R, C> ,
                        camera: &Camera, projection: [[f32; 4]; 4],
                        light_manager: &mut e_lights_manager::LightManager,
                        model_manager: &mut e_model_manager::ModelManager<R>,)
    where   R: gfx::Resources,
            C: gfx::CommandBuffer<R>,
    {

        //Clean first for each model
        for (name, model) in &model_manager.models {
            encoder.clear(&model.data.out_color, CLEAR_COLOR);
            encoder.clear_depth(&model.data.out_depth, 1.0);
        }

        //Create a Vec<e_light::TYPE> which holds the current lights
        // All not used slots will be marked inactive and will be discarded in
        //the light calculation
        let mut active_dir_slots = 0;
        let mut active_spot_slots = 0;
        let mut active_point_slots = 0;

        //Vecs which hold all lights, including the inacive ones, should always be
        //the same size as max_TYPE_size in LightOptions of LightManager
        let mut current_dir_lights: Vec<g_object::Light_Directional> = Vec::new();
        let mut current_spot_lights: Vec<g_object::Light_Spot> = Vec::new();
        let mut current_point_lights: Vec<g_object::Light_Point> = Vec::new();

        //Directional lights
        {
            //Push active ones
            for (name, light) in &light_manager.directional_lights {
                current_dir_lights.push(g_object::Light_Directional {
                                                d_lightDirection: light.d_lightDirection ,
                                                d_lightColor: light.d_lightColor,
                                                d_lightStrength: light.d_lightStrength,
                                                _pad1: 1.0,
                                                _pad2: 1.0,
                                                d_active: true,
                                            });
                //Add active index
                active_dir_slots += 1;
            }

            //println!("INFO: Pushed {} of {} directional lights", active_dir_slots, light_manager.light_settings.max_dir_lights);

        }

        //Spot lights
        {
            //Push active ones
            for (name, light) in &light_manager.spot_lights {
                current_spot_lights.push(g_object::Light_Spot {
                                                s_lightPos: light.s_lightPos,
                                                s_lightDirection: light.s_lightDirection,
                                                s_lightColor: light.s_lightColor,
                                                s_cutOff: light.s_cutOff,
                                                s_outerCutOff: light.s_outerCutOff,
                                                s_constant: light.s_constant,
                                                s_linear: light.s_linear,
                                                s_quadratic: light.s_quadratic,
                                                _pad1: 1.0,
                                                _pad2: 1.0,
                                                //_pad3: 1.0,
                                                s_active: true,
                                            });
                //Add active index
                active_spot_slots += 1;
            }

            //println!("INFO: Pushed {} of {} spot lights", active_spot_slots, light_manager.light_settings.max_spot_lights);

        }

        //Point lights
        {
            //Push active ones
            for (name, light) in &light_manager.point_lights {
                current_point_lights.push(g_object::Light_Point {
                                                p_lightPos: light.p_lightPos,
                                                p_lightColor: light.p_lightColor,
                                                p_constant: light.p_constant,
                                                p_linear: light.p_linear,
                                                p_quadratic: light.p_quadratic,
                                                p_lightStrength: light.p_lightStrength,
                                                _pad1: 1.0,
                                                _pad2: 1.0,
                                                _pad3: 1.0,
                                                p_active: true,
                                            });
                //Add active index
                active_point_slots += 1;
            }

            //println!("INFO: Pushed {} of {} point lights", active_point_slots, light_manager.light_settings.max_point_lights);

        }

        //Render each (active) model
        for (name, model) in &model_manager.models {

            //Process only active models
            if model.is_active{

                //Prepare transform
                let l_transform = {
                                    Matrix4::from_translation(model.world_location) *
                                    (Matrix4::from_angle_x(Rad::from(Deg(model.world_rotation.x))) *
                                    Matrix4::from_angle_y(Rad::from(Deg(model.world_rotation.y))) *
                                    Matrix4::from_angle_z(Rad::from(Deg(model.world_rotation.z)))) *
                                    Matrix4::from_nonuniform_scale(model.world_scale.x, model.world_scale.y, model.world_scale.z)
                                };


                //Transform
                let locals = g_object::Locals { transform: l_transform.into(),
                                                projection: projection,
                                                view: camera.return_view_matrix()
                                            };

                //Changed the max settings to the currently in use lights
                let light_info_pass = g_object::Light_Info {max_dir_lights: active_dir_slots,
                                                            max_spot_lights: active_spot_slots,
                                                            max_point_lights: active_point_slots};



                encoder.update_buffer(&model.data.dir_light, &current_dir_lights[..], 0);
                encoder.update_buffer(&model.data.spot_light, &current_spot_lights[..], 0);
                encoder.update_buffer(&model.data.point_light, &current_point_lights[..], 0);

                //Material Properties
                let material = g_object::Material { shininess: model.material.shininess,
                                                    ambient: model.material.ambient,
                                                    diffuse_intensity: model.material.diffuse_intensity,
                                                    specular: model.material.specular_instensity};

                //Camera
                let camera = g_object::Camera { viewPos: camera.cameraPos.extend(1.0).into()};


                encoder.update_constant_buffer(&model.data.locals, &locals);

                encoder.update_constant_buffer(&model.data.light_info, &light_info_pass);

                encoder.update_constant_buffer(&model.data.material, &material);

                encoder.update_constant_buffer(&model.data.camera, &camera);

                encoder.draw(&model.slices, &model.pso, &model.data);
            }
        }
    }

}
