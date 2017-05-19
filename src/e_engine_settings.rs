//Settings from which opengl context
//and the engine will be created


pub struct EngineSettings {

    //Window
    pub name: String,
    pub width: u32,
    pub height: u32,

    //Light
    pub max_dir_lights: u32,
    pub max_spot_ligths: u32,
    pub max_point_lights: u32,
}


impl EngineSettings {
    pub fn new()-> Self {
        //Return a default window
        EngineSettings {    name: String::from("Engine Window"),
                            width: 1280,
                            height: 720,

                            max_dir_lights: 1,
                            max_spot_ligths: 1,
                            max_point_lights: 6}

    }

    pub fn with_dimensions(mut self, width: u32, height: u32) -> Self {
        self.height = height;
        self.width = width;
        self
    }

    pub fn with_light_counts(mut self, max_dir_lights: u32, max_spot_ligths: u32, max_point_lights: u32) -> Self {
        self.max_dir_lights = max_dir_lights;
        self.max_point_lights = max_point_lights;
        self.max_spot_ligths = max_spot_ligths;
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = String::from(name);
        self
    }
}
