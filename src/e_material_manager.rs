use std::collections::HashMap;


use e_material;
use gfx;

pub struct MaterialManager<R: gfx::Resources> {
    pub materials: HashMap<String, e_material::Material<R>>,
}


impl <R: gfx::Resources> MaterialManager<R> {
    pub fn new() -> Self {
        MaterialManager { materials: HashMap::new()}
    }

    pub fn add<F>(&mut self, mut factory: &mut F, name: &str, diffuse_path: &str, specular_path: &str, normal_path: &str)
    where F: gfx::Factory<R>
    {
        let mut new_material = e_material::Material::new::<F>(&mut factory);
        new_material.set_textures::<F>(&mut factory, diffuse_path, specular_path, normal_path);
        self.materials.insert(String::from(name), new_material);
    }

    pub fn get_material(&mut self, name: &str) -> Option<&e_material::Material<R>> {
        self.materials.get(&String::from(name))
    }
}
