//The scene manager, responsible for loadeing/ unloading the different
//Scenes
use std::collections::HashMap;
use gfx;

use e_scene;
use e_engine_settings;

pub struct SceneManger<R: gfx::Resources> {
    Scenes: HashMap<String, e_scene::Scene<R>>,
}


impl<R: gfx::Resources> SceneManger<R> {
    pub fn new(engine_setting: &e_engine_settings::EngineSettings) -> Self{

        let default_scene = e_scene::Scene::new("Default_Scene", &engine_setting);


        let mut manager = SceneManger{Scenes: HashMap::new()};
        manager.Scenes.insert(String::from("Default_Scene"), default_scene);
        manager
    }

    pub fn add(&mut self, name: &str, engine_setting: &e_engine_settings::EngineSettings)
    {
        let scene_to_add =  e_scene::Scene::new(name, engine_setting);
        self.Scenes.insert(String::from(name.clone()), scene_to_add);
    }

    //Returns the scene by name or the default scene if not found
    pub fn get_scene(&mut self, name: &str) -> &mut e_scene::Scene<R> {

        //If the key is in the hash map it is save to unwrap the value
        if self.Scenes.contains_key(&String::from(name)){
            self.Scenes.get_mut(&String::from(name)).unwrap()
        }else {
            self.Scenes.get_mut(&String::from("Default_Scene")).unwrap()
        }
    }
}
