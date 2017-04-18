

//Use HashMap for model management
use std::collections::HashMap;


use g_object;

use gfx;
use gfx_window_glutin;
use tobj;

use gfx::traits::FactoryExt;
use gfx::{Resources, Bundle, texture, Device};

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

}
