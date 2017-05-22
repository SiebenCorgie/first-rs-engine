

//Use HashMap for model management
use std::collections::HashMap;
use std::path::Path;
//Threading
use std::thread;
use std::sync::mpsc;


use g_object;
use g_camera::*;

use gfx;
use gfx_window_glutin;

use gfx::traits::FactoryExt;
use gfx::{Resources, Bundle, texture, Device};
use cgmath::*;

use t_assimp_importer;
use e_material;
use e_material_manager;
use e_light;
use e_lights_manager;

//Needed for initing the object
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

    //import a model via the assimp importer (supported)
    //Change to a multi core approach

    pub fn import_model_assimp<F> (&mut self, name: &str, path: &str,
                        factory: &mut F,
                        main_color: &mut gfx::handle::RenderTargetView<R, ColorFormat>,
                        main_depth: &mut gfx::handle::DepthStencilView<R, DepthFormat>,
                        mut material: &mut e_material::Material,
                        material_type: g_object::MaterialType,
                        light_manager: &e_lights_manager::LightManager)
        where F: gfx::Factory<R>,
        {
            let importer = t_assimp_importer::Importer::new();
            let (mesh_vec, indice_vec, name_vec) = importer.import_mesh(path);

            //Add each mesh individual
            for i in 0..mesh_vec.len(){
                let final_name: String = String::from(name) + &"_" + &name_vec[i];
                self.add(final_name, g_object::Object::new(factory, main_color, main_depth,
                                                            mesh_vec[i].clone(), indice_vec[i].clone(),
                                                            &mut material, material_type.clone(), light_manager));
            }
    }
    /*
    //New assimp importer
    pub fn import_model_assimp<F> (&mut self, name: &str, path: &str,
                        factory: &mut F,
                        main_color: &mut gfx::handle::RenderTargetView<R, ColorFormat>,
                        main_depth: &mut gfx::handle::DepthStencilView<R, DepthFormat>,
                        mut material: &mut e_material::Material,
                        material_type: g_object::MaterialType,
                        light_manager: &e_lights_manager::LightManager)
        where F: gfx::Factory<R>,
        {
            let importer = t_assimp_importer::Importer::new();


            let (tx, rx) = mpsc::channel();
            //Import on another thread
            thread::spawn(move || {
                let (mesh_vec, indice_vec, name_vec) = importer.import_mesh(path);
                //Sending to main thread
                tx.send(mesh_vec, indice_vec, name_vec).unwrap();
            });

            let received = rx.try_recv();
            match received {
                Ok(mesh_vec, indice_vec, name_vec) => {
                    println!("Got data!");
                    //Add each mesh individual
                    for i in 0..mesh_vec.len(){
                        let final_name: String = String::from(name) + &"_" + &name_vec[i];
                        self.add(final_name, g_object::Object::new(factory, main_color, main_depth,
                                                                    mesh_vec[i].clone(), indice_vec[i].clone(),
                                                                    &mut material, material_type.clone(), light_manager));
                    }
                },
                Err(msk) => println!("Still waiting: {}", msk),
            }



    }
    */

    //Print all objects in the scene
    pub fn print_scene(&self){
        println!("Scene:");
        for (name, model) in &self.models {
            println!("Object: {}", name);
        }
    }

    //Get a model by name from the model manager. Returns a mutable reference to this object.
    //Might have to change from panic to a "if none default model" style
    pub fn get_model(&mut self, name: &str) -> &mut g_object::Object<R> {

        let give_back = self.models.get_mut(&String::from(name));
        //panic if it went wrong
        if give_back.is_none(){panic!("[{}] not found!   ", name);}
        let result = give_back.unwrap();
        result
    }

    //Tests if a model (by name) is in the manager
    pub fn is_in_manager(&mut self, name: &str) -> bool {
        if self.models.contains_key(&String::from(name)){
            true
        }else {
            false
        }

    }

}
