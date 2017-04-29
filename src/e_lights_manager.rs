
//The light manager


use e_light;
use std::collections::HashMap;


struct LightOptions {
        max_lights: u32,
}


struct LightManager {
    light_settings: LightOptions,
    directional_lights: HashMap<String, e_light::Light_Directional>,
    spot_lights: HashMap<String, e_light::Light_Spot>,
    point_lights: HashMap<String, e_light::Light_Point>,
}


//Create a new LightOption for passing to the Light Manager
impl LightOptions {
    pub fn new(max_lights: u32) -> Self {
        LightOptions {max_lights: max_lights}
    }
}


impl LightManager {

    //Create the light manager with its settings
    pub fn new(max_lights: u32) -> Self {
        let light_options = LightOptions::new(max_lights);

        LightManager {  light_settings: light_options, directional_lights: HashMap::new(),
                        spot_lights: HashMap::new(), point_lights: HashMap::new()}
    }

    //Add_er for the lights
    pub fn add_directional_light(&mut self, name: &str, dir_light: e_light::Light_Directional) {
        self.directional_lights.insert(String::from(name), dir_light);
    }

    pub fn add_spot_light(&mut self, name: &str, spot_light: e_light::Light_Spot) {
        self.spot_lights.insert(String::from(name), spot_light);
    }

    pub fn add_point_light(&mut self, name: &str, point_light: e_light::Light_Point) {
        self.point_lights.insert(String::from(name), point_light);
    }


    //Get_er
    pub fn get_directional_light(&mut self, name: &str) ->Option<&mut e_light::Light_Directional> {
        self.directional_lights.get_mut(&String::from(name))
    }

    pub fn get_spot_light(&mut self, name: &str) ->Option<&mut e_light::Light_Spot> {
        self.spot_lights.get_mut(&String::from(name))
    }

    pub fn get_point_light(&mut self, name: &str) ->Option<&mut e_light::Light_Point> {
        self.point_lights.get_mut(&String::from(name))
    }

}
