

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

    pub fn render<C>(&self, encoder: &mut gfx::Encoder<R, C> ,
                        camera: &g_camera::Camera, projection: [[f32; 4]; 4],)
    where   C: gfx::CommandBuffer<R>,
    {

        //Clean
        for (name, model) in &self.models {
            encoder.clear(&model.data.out_color, CLEAR_COLOR);
            encoder.clear_depth(&model.data.out_depth, 1.0);
        }

        //Render
        for (name, model) in &self.models {

            //Transform
            let locals = g_object::Locals { transform: Matrix4::from_translation(model.world_location).into(),
                                            projection: projection,
                                            view: camera.return_view_matrix()
                                        };



            //Light properties
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


            //Material Properties
            let material = g_object::Material { shininess: model.material.shininess,
                                                ambient: model.material.ambient,};

            //Camera
            let camera = g_object::Camera { viewPos: camera.cameraPos.extend(1.0).into()};


            encoder.update_constant_buffer(&model.data.locals, &locals);

            encoder.update_constant_buffer(&model.data.dir_light, &light_dir);
            encoder.update_constant_buffer(&model.data.spot_light, &light_spot);
            encoder.update_constant_buffer(&model.data.point_light, &light_point);

            encoder.update_constant_buffer(&model.data.material, &material);

            encoder.update_constant_buffer(&model.data.camera, &camera);

            encoder.draw(&model.slices, &model.pso, &model.data);
        }
    }

    pub fn import_model<F> (&mut self, name: &str, path: &str,
                        factory: &mut F,
                        main_color: &mut gfx::handle::RenderTargetView<R, ColorFormat>,
                        main_depth: &mut gfx::handle::DepthStencilView<R, DepthFormat>,
                        mut material: &mut e_material::Material)
        where F: gfx::Factory<R>,
        {

        let importer = t_obj_importer::Importer::new();
        let (mesh_vec, indice_vec, name_vec) = importer.import_mesh(path);

        //Add each mesh individual
        for i in 0..mesh_vec.len(){
            let final_name: String = String::from(name) + &"_" + &name_vec[i];
            self.add(final_name, g_object::Object::new(factory, main_color, main_depth, mesh_vec[i].clone(), indice_vec[i].clone(), &mut material));
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
