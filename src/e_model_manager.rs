

//Use HashMap for model management
use std::collections::HashMap;
use std::path::Path;


use g_object;
use g_camera;

use gfx;
use gfx_window_glutin;
use tobj;

use gfx::traits::FactoryExt;
use gfx::{Resources, Bundle, texture, Device};
use cgmath::*;

use t_obj_importer;
use e_material;
use e_material_manager;
use e_light;
use e_lights_manager;

const CLEAR_COLOR: [f32; 4] = [0.05, 0.05, 0.1, 1.0];
pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;



pub struct ModelManager<R: Resources> {
    pub models: HashMap<String, g_object::Object<R>>,
}


impl<R: gfx::Resources> ModelManager<R> {

    pub fn new() -> Self {
        ModelManager { models: HashMap::new() }
    }

    pub fn add(&mut self, name: String, object: g_object::Object<R>){
        self.models.insert(name,object);
    }

    pub fn render<C>(   &self, encoder: &mut gfx::Encoder<R, C> ,
                        camera: &g_camera::Camera, projection: [[f32; 4]; 4],
                        light_manager: &mut e_lights_manager::LightManager)
    where   C: gfx::CommandBuffer<R>,
    {

        //Clean
        for (name, model) in &self.models {
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
                                                d_active: true,
                                            });
                //Add active index
                active_dir_slots += 1;
            }

            //Push inactive till reaching the max lights
            for index in active_dir_slots..light_manager.light_settings.max_dir_lights {
                current_dir_lights.push(g_object::Light_Directional {
                                                d_lightDirection: Vector4::new(10.0, -10.0, 10.0, 1.0).into(),
                                                d_lightColor: Vector4::new(1.0, 1.0, 1.0, 1.0).into(),
                                                d_lightStrength: 1.0,
                                                d_active: false,
                                            });
                //Add deactive index
                active_dir_slots += 1;
            }
            println!("INFO: Pushed {} of {} directional lights", active_dir_slots, light_manager.light_settings.max_dir_lights);
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
                                                s_active: true,
                                            });
                //Add active index
                active_spot_slots += 1;
            }

            //Push inactive till reaching the max lights
            for index in active_spot_slots..light_manager.light_settings.max_spot_lights {
                current_spot_lights.push(g_object::Light_Spot {
                                                s_lightPos: Vector4::new(10.0, 10.0, 10.0, 1.0).into(),
                                                s_lightDirection: Vector4::new(10.0, -10.0, 10.0, 1.0).into(),
                                                s_lightColor: Vector4::new(1.0, 1.0, 1.0, 1.0).into(),
                                                s_cutOff: 45.0,
                                                s_active: false,
                                            });
                //Add deactive index
                active_spot_slots += 1;
            }
            println!("INFO: Pushed {} of {} spot lights", active_spot_slots, light_manager.light_settings.max_spot_lights);
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
                                                p_active: true,
                                            });
                //Add active index
                active_point_slots += 1;
            }

            //Push inactive till reaching the max lights
            for index in active_point_slots..light_manager.light_settings.max_point_lights {
                current_point_lights.push(g_object::Light_Point {
                                                p_lightPos: Vector4::new(2.0, 2.0, 2.0, 1.0).into(),
                                                p_lightColor: Vector4::new(1.0, 0.95, 0.95, 1.0).into(),
                                                p_constant: 1.0,
                                                p_linear: 0.09,
                                                p_quadratic: 0.032,
                                                p_lightStrength: 1.0,
                                                p_active: false,
                                            });
                //Add deactive index
                active_point_slots += 1;
            }
            println!("INFO: Pushed {} of {} point lights", active_point_slots, light_manager.light_settings.max_point_lights);
        }

        //Render
        for (name, model) in &self.models {

            //Transform
            let locals = g_object::Locals { transform: Matrix4::from_translation(model.world_location).into(),
                                            projection: projection,
                                            view: camera.return_view_matrix()
                                        };

            let light_info_pass = g_object::Light_Info {max_dir_lights: light_manager.light_settings.max_dir_lights as i32,
                                                        max_spot_lights: light_manager.light_settings.max_spot_lights as i32,
                                                        max_point_lights: light_manager.light_settings.max_point_lights as i32};



            /*Light properties
            let light_dir = g_object::Light_Directional {
                                            d_lightDirection: Vector4::new(10.0, -10.0, 10.0, 1.0).into(),
                                            d_lightColor: Vector4::new(1.0, 1.0, 1.0, 1.0).into(),
                                            d_lightStrength: 1.0,
                                        };
            let light_spot = g_object::Light_Spot {
                                            s_lightPos: Vector4::new(10.0, 10.0, 10.0, 1.0).into(),
                                            s_lightDirection: Vector4::new(10.0, -10.0, 10.0, 1.0).into(),
                                            s_lightColor: Vector4::new(1.0, 1.0, 1.0, 1.0).into(),
                                            s_cutOff: 45.0,
                                        };

            let light_point = g_object::Light_Point {
                                            l_lightPos: Vector4::new(2.0, 2.0, 2.0, 1.0).into(),
                                            l_lightColor: Vector4::new(1.0, 0.95, 0.95, 1.0).into(),
                                            l_constant: 1.0,
                                            l_linear: 0.09,
                                            l_quadratic: 0.032,
                                            l_lightStrength: 1.0,
                                        };
            */
            //New light passing
            for dir_light in 0..current_dir_lights.len() {
                encoder.update_buffer(&model.data.dir_light, &[current_dir_lights[dir_light]], dir_light);
            }

            for spot_light in 0..current_spot_lights.len() {
                encoder.update_buffer(&model.data.spot_light, &[current_spot_lights[spot_light]], spot_light);
            }

            for point_light in 0..current_point_lights.len() {
                encoder.update_buffer(&model.data.point_light, &[current_point_lights[point_light]], point_light);
            }


            //Material Properties
            let material = g_object::Material { shininess: model.material.shininess,
                                                ambient: model.material.ambient,};

            //Camera
            let camera = g_object::Camera { viewPos: camera.cameraPos.extend(1.0).into()};


            encoder.update_constant_buffer(&model.data.locals, &locals);

            encoder.update_constant_buffer(&model.data.light_info, &light_info_pass);

            encoder.update_constant_buffer(&model.data.material, &material);

            encoder.update_constant_buffer(&model.data.camera, &camera);

            encoder.draw(&model.slices, &model.pso, &model.data);
        }
    }

    pub fn import_model<F> (&mut self, name: &str, path: &str,
                        factory: &mut F,
                        main_color: &mut gfx::handle::RenderTargetView<R, ColorFormat>,
                        main_depth: &mut gfx::handle::DepthStencilView<R, DepthFormat>,
                        mut material: &mut e_material::Material,
                        light_manager: &e_lights_manager::LightManager)
        where F: gfx::Factory<R>,
        {

        let importer = t_obj_importer::Importer::new();
        let (mesh_vec, indice_vec, name_vec) = importer.import_mesh(path);

        //Add each mesh individual
        for i in 0..mesh_vec.len(){
            let final_name: String = String::from(name) + &"_" + &name_vec[i];
            self.add(final_name, g_object::Object::new(factory, main_color, main_depth,
                                                        mesh_vec[i].clone(), indice_vec[i].clone(),
                                                        &mut material, light_manager));
        }
    }

    pub fn print_scene(&self){
        for (name, model) in &self.models {
            println!("Name: {}", name);
        }
    }

    pub fn get_model(&mut self, name: &str) -> &mut g_object::Object<R> {

        let give_back = self.models.get_mut(&String::from(name));
        //panic if it went wrong
        if give_back.is_none(){panic!("[{}] not found!   ", name);}
        let result = give_back.unwrap();
        result
    }

    pub fn is_in_manager(&mut self, name: &str) -> bool {
        let give_back = self.models.get_mut(&String::from(name));

        let mut result = false;
        if give_back.is_some(){ result = true; }

        result

    }

}
