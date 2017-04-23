
use gfx;
use image;

pub struct Material<R: gfx::Resources> {
    diffuse: gfx::handle::ShaderResourceView<R, [f32; 4]>,
    specular: gfx::handle::ShaderResourceView<R, [f32; 4]>,
    normal: gfx::handle::ShaderResourceView<R, [f32; 4]>,
    ambient: f32,
    shininess: f32,
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


impl<R: gfx::Resources> Material<R>{
    pub fn new<F>(mut factory: &mut F) -> Self
    where F: gfx::Factory<R>
    {
        let diffuse_map = gfx_load_texture::<F,R>(&mut factory, "data/Textures/fallback_diff.png");
        let specular_map = gfx_load_texture::<F,R>(&mut factory, "data/Textures/fallback_spec.png");
        let normal_map = gfx_load_texture::<F,R>(&mut factory, "data/Textures/fallback_nrm.png");

        Material{ ambient: 0.1, shininess: 0.5, diffuse: diffuse_map, specular: specular_map, normal: normal_map}
    }

    pub fn set_textures<F>(&mut self, mut factory: &mut F, diffuse_path: &str, specular_path: &str, normal_path: &str)
    where F: gfx::Factory<R>
    {
        //Set all given textures
        if diffuse_path != "_"{
            self.diffuse = gfx_load_texture::<F,R>(&mut factory, diffuse_path);
        }

        if specular_path != "_"{
            self.specular = gfx_load_texture::<F,R>(&mut factory, specular_path);
        }

        if normal_path != "_"{
            self.normal = gfx_load_texture::<F,R>(&mut factory, normal_path);
        }
    }
}
