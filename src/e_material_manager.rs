use std::collections::HashMap;


use e_material;
use gfx;

pub struct MaterialManager {
    pub materials: HashMap<String, e_material::Material>,
}


impl MaterialManager {
    pub fn new() -> Self{
        //Create a default material as fallback
        let new_material = e_material::Material::new();

        //The hash map of the MaterialManager
        let mut materials = MaterialManager { materials: HashMap::new()};
        //insert default
        materials.materials.insert(String::from("default"), new_material);
        //Create the material manager with first default material
        materials
    }

    pub fn add(&mut self, name: &str, diffuse_path: &str, specular_path: &str, normal_path: &str)
    {
        let mut new_material = e_material::Material::new();
        new_material.set_textures(diffuse_path, specular_path, normal_path);

        //Prevents from adding a new default
        if name != "default"{
            self.materials.insert(String::from(name), new_material);
        } else{
            self.materials.insert((String::from(name) + "_new"), new_material);
        }
    }

    pub fn get_material(&mut self, name: &str) -> &mut e_material::Material {
        self.materials.get_mut(&String::from(name)).unwrap()
    }
}
