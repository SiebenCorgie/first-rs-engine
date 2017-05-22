// Descibes a scene and its components
use gfx;
use e_model_manager;
use g_camera;
use g_camera::CameraTyp;


use e_material_manager;
use e_lights_manager;
use e_engine_settings;

pub struct Scene<R: gfx::Resources> {
    pub name: String,
    pub camera: g_camera::Camera,
    pub material_manager: e_material_manager::MaterialManager,
    pub light_manager: e_lights_manager::LightManager,
    pub model_manager: e_model_manager::ModelManager<R>,

}

impl<R: gfx::Resources> Scene<R> {
    pub fn new(scene_name: &str, engine_settings: &e_engine_settings::EngineSettings) -> Self {
        //Init all managers
        let camera: g_camera::Camera = g_camera::Camera::new();
        let material_manager: e_material_manager::MaterialManager = e_material_manager::MaterialManager::new();
        let light_manager: e_lights_manager::LightManager = e_lights_manager::LightManager::new(
            engine_settings.max_dir_lights,
            engine_settings.max_spot_ligths,
            engine_settings.max_point_lights);
        let model_manager: e_model_manager::ModelManager<R> = e_model_manager::ModelManager::new();

        Scene{ name: String::from(scene_name),
            camera: camera,
            material_manager: material_manager,
            light_manager: light_manager,
            model_manager: model_manager,}
    }
}
