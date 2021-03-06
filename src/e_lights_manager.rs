
//The light manager


use e_light;
use std::collections::HashMap;


pub struct LightOptions {
        pub max_dir_lights: u32,
        pub max_spot_lights: u32,
        pub max_point_lights: u32,

        pub current_dir_lights: u32,
        pub current_spot_lights: u32,
        pub current_point_lights: u32,
}


pub struct LightManager {
    pub light_settings: LightOptions,
    pub directional_lights: HashMap<String, e_light::Light_Directional>,
    pub spot_lights: HashMap<String, e_light::Light_Spot>,
    pub point_lights: HashMap<String, e_light::Light_Point>,
}


//Create a new LightOption for passing to the Light Manager
impl LightOptions {
    pub fn new(max_directional_lights: u32, max_spot_lights: u32, max_point_lights: u32) -> Self {
        LightOptions {  max_dir_lights: max_directional_lights,
                        max_spot_lights: max_spot_lights,
                        max_point_lights: max_point_lights,
                        current_dir_lights: 0,
                        current_spot_lights: 0,
                        current_point_lights: 0}
    }
}


impl LightManager {

    //Create the light manager with its settings
    pub fn new(max_directional_lights: u32, max_spot_lights: u32, max_point_lights: u32) -> Self {
        let light_options = LightOptions::new(max_directional_lights, max_spot_lights, max_point_lights);

        LightManager {  light_settings: light_options, directional_lights: HashMap::new(),
                        spot_lights: HashMap::new(), point_lights: HashMap::new()}
    }

    //Add_er for the lights
    pub fn add_directional_light(&mut self, name: &str, dir_light: e_light::Light_Directional) {
        //Add the lights if the space is not full else, discard
        if self.light_settings.current_dir_lights != self.light_settings.max_dir_lights {
            self.directional_lights.insert(String::from(name), dir_light);
            self.light_settings.current_dir_lights += 1;
        } else {
            println!("ERROR: Already reach the maximum of {} directional lights!", self.light_settings.max_dir_lights);
        }
    }

    pub fn add_spot_light(&mut self, name: &str, spot_light: e_light::Light_Spot) {
        //Add the lights if the space is not full else, discard
        if self.light_settings.current_spot_lights != self.light_settings.max_spot_lights {
            self.spot_lights.insert(String::from(name), spot_light);
            self.light_settings.current_spot_lights += 1;
        } else {
            println!("ERROR: Already reach the maximum of {} spot lights!", self.light_settings.max_spot_lights);
        }
    }

    pub fn add_point_light(&mut self, name: &str, point_light: e_light::Light_Point) {
        //Add the lights if the space is not full else, discard
        if self.light_settings.current_point_lights != self.light_settings.max_point_lights {
            self.point_lights.insert(String::from(name), point_light);
            self.light_settings.current_point_lights += 1;
        } else {
            println!("ERROR: Already reach the maximum of {} point lights!", self.light_settings.max_point_lights);
        }
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
