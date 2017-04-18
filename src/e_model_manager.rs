
use g_object;


use gfx;
use gfx_window_glutin;
use tobj;

use gfx::traits::FactoryExt;
use gfx::{Resources, Bundle, texture, Device};

pub struct ModelManager<R: Resources> {
    pub models: Vec<g_object::Object<R>>,
}


impl<R: gfx::Resources> ModelManager<R> {

    pub fn new() -> Self {
        let objec_vector: Vec<g_object::Object<R>> = Vec::new();
        ModelManager { models: objec_vector }
    }

    pub fn add_object(&mut self, add_object: g_object::Object<R>){

        self.models.push(add_object);


    }

}
