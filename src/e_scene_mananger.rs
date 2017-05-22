//The scene manager, responsible for loadeing/ unloading the different
//Scenes
use std::collections::HashMap;
use gfx;

use e_scene;
use e_engine_settings;

pub struct SceneManger<R: gfx::Resources> {
    scenes: HashMap<String, e_scene::Scene<R>>,
    active_scene: String,
}


impl<R: gfx::Resources> SceneManger<R> {
    pub fn new(engine_setting: &e_engine_settings::EngineSettings) -> Self{

        let default_scene = e_scene::Scene::new("Default_Scene", &engine_setting);

        let mut manager = SceneManger{scenes: HashMap::new(), active_scene: String::from("Default_Scene")};
        manager.scenes.insert(String::from("Default_Scene"), default_scene);
        manager
    }

    pub fn add(&mut self, name: &str, engine_setting: &e_engine_settings::EngineSettings)
    {
        let scene_to_add =  e_scene::Scene::new(name, engine_setting);
        self.scenes.insert(String::from(name.clone()), scene_to_add);
    }

    //Returns the scene by name or the default scene if not found
    pub fn get_scene(&mut self, name: &str) -> &mut e_scene::Scene<R> {

        //If the key is in the hash map it is save to unwrap the value
        if self.scenes.contains_key(&String::from(name)){
            self.scenes.get_mut(&String::from(name)).unwrap()
        }else {
            println!("Scene '{}' not found, returning default scene", name);
            self.scenes.get_mut(&String::from("Default_Scene")).unwrap()
        }
    }

    pub fn set_active(&mut self, name: &str){
        if self.scenes.contains_key(&String::from(name)){
            self.active_scene = String::from(name);
        }else{
            println!("Sorry scene '{}' not found in the scene map \n you might have to add this scene first", name);
        }
    }

    pub fn get_active(&mut self) -> &mut e_scene::Scene<R>{
        self.scenes.get_mut(&self.active_scene.to_string()).unwrap()
    }
}
