
use gfx;
use image;
#[derive(Clone)]
pub struct Material {
    pub diffuse: String,
    pub specular: String,
    pub normal: String,
    pub ambient: f32,
    pub shininess: f32,
}


fn gfx_load_texture<F, R>(factory: &mut F, path: &str) -> gfx::handle::ShaderResourceView<R, [f32; 4]>
    where F: gfx::Factory<R>,
          R: gfx::Resources,
{
    use gfx::format::Rgba8;
    let img = image::open(path).unwrap().to_rgba();
    let (width, height) = img.dimensions();
    let kind = gfx::texture::Kind::D2(width as u16, height as u16, gfx::texture::AaMode::Single);
    let (_, view) = factory.create_texture_immutable_u8::<Rgba8>(kind, &[&img]).unwrap();
    view
}


impl Material{
    pub fn new() -> Self
    {

        Material{ ambient: 0.1, shininess: 32.0,
                diffuse: String::from("data/Textures/fallback_diff.png"),
                specular: String::from("data/Textures/fallback_spec.png"),
                normal: String::from("data/Textures/fallback_nrm.png")}
    }

    pub fn set_textures(&mut self, diffuse_path: &str, specular_path: &str, normal_path: &str)
    {
        //Set all given textures
        if diffuse_path != "_"{
            self.diffuse = String::from(diffuse_path);
        }

        if specular_path != "_"{
            self.specular = String::from(specular_path);
        }

        if normal_path != "_"{
            self.normal = String::from(normal_path);
        }
    }

    pub fn set_shininess(&mut self, new: f32){
        self.shininess = new;
    }

    pub fn set_ambient(&mut self, new: f32){
        self.ambient = new;
    }
}
