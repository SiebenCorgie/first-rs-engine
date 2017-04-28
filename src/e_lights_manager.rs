
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
