

//Use HashMap for model management
use std::collections::HashMap;


use g_object;
use g_camera;

use gfx;
use gfx_window_glutin;
use tobj;

use gfx::traits::FactoryExt;
use gfx::{Resources, Bundle, texture, Device};
use cgmath::*;

const CLEAR_COLOR: [f32; 4] = [0.5, 0.5, 1.0, 1.0];



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

    pub fn render<C>(&self, encoder: &mut gfx::Encoder<R, C> , camera: &g_camera::Camera, projection: [[f32; 4]; 4])
    where C: gfx::CommandBuffer<R>
    {

        //Clean
        for (name, model) in &self.models {
            encoder.clear(&model.data.out_color, CLEAR_COLOR);
            encoder.clear_depth(&model.data.out_depth, 1.0);
        }

        //Render
        for (name, model) in &self.models {
            let locals = g_object::Locals { transform: Matrix4::from_translation(model.world_location).into(),
                                            projection: projection,
                                            view: camera.return_view_matrix()};

            encoder.update_constant_buffer(&model.data.locals, &locals);
            encoder.draw(&model.slices, &model.pso, &model.data);
        }
    }

}
