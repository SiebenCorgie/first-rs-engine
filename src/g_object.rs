use image;
use cgmath;
#[macro_use]
use gfx;
use gfx_window_glutin;

use gfx::traits::FactoryExt;
use gfx::*;

use e_material;
use e_lights_manager;

use cgmath::*;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 3] = "a_Pos",
        tex_coord: [f32; 2] = "a_TexCoord",
        normal: [f32; 3] = "a_Normal",
        tangent: [f32; 3] = "a_Tangent",
        color: [f32; 3] = "a_Color",
    }
    constant Locals {
        transform: [[f32;4];4] = "u_Model_Transform",
        projection: [[f32;4];4] = "u_Projection",
        view: [[f32;4];4] = "u_View",
    }
    constant Light_Directional {
        d_lightDirection: [f32; 4] = "d_lightDir",
        d_lightColor: [f32; 4] = "d_lightColor",
        d_lightStrength: f32 = "d_lightStrength",
        _pad1: f32 = "_pad1",
        _pad2: f32 = "_pad2",
        d_active: bool = "d_active",

    }
    constant Light_Spot {
        s_lightPos: [f32; 4] = "s_lightPos",
        s_lightDirection: [f32; 4] = "s_lightDirection",
        s_lightColor: [f32; 4] = "s_lightColor",
        s_cutOff: f32 = "s_cutOff",
        s_outerCutOff: f32 = "s_outerCutOff",
        s_constant: f32 = "s_constant",
        s_linear: f32 = "s_linear",
        s_quadratic: f32 = "s_quadratic",
        _pad1: f32 = "_pad1",
        _pad2: f32 = "_pad2",
        //_pad3: f32 = "_pad3",
        s_active: bool = "s_active",

    }
    constant Light_Point {
        p_lightPos: [f32; 4] = "p_lightPos",
        p_lightColor: [f32; 4] = "p_lightColor",
        p_constant: f32 = "p_constant",
        p_linear: f32 = "p_linear",
        p_quadratic: f32 = "p_quadratic",
        p_lightStrength: f32 = "p_lightStrength",
        _pad1: f32 = "_pad1",
        _pad2: f32 = "_pad2",
        _pad3: f32 = "_pad3",
        p_active: bool = "p_active",
    }

    constant Light_Info {
        max_dir_lights: i32 = "max_dir_lights",
        max_spot_lights: i32 = "max_spot_lights",
        max_point_lights: i32 = "max_point_lights",
    }

    constant Camera {
        viewPos: [f32; 4] = "c_viewPos",
    }

    constant Material {
        shininess: f32 = "shininess",
        ambient: f32 = "ambient",
        diffuse_intensity: f32 = "diffuse_intensity",
        specular: f32 = "specular",
    }

    //Cube Pipeline
    pipeline my_pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",

        dir_light: gfx::ConstantBuffer<Light_Directional> = "Light_Directional",
        spot_light: gfx::ConstantBuffer<Light_Spot> = "Light_Spot",
        point_light: gfx::ConstantBuffer<Light_Point> = "Light_Point",

        light_info: gfx::ConstantBuffer<Light_Info> = "Light_Info",

        material: gfx::ConstantBuffer<Material> = "Material",

        camera: gfx::ConstantBuffer<Camera> = "Camera",

        diffuse_tex: gfx::TextureSampler<[f32; 4]> = "t_Diffuse",
        specular: gfx::TextureSampler<[f32; 4]> = "t_Specular",
        normal: gfx::TextureSampler<[f32; 4]> = "t_Normal",

        out_color: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA),
        out_depth: gfx::DepthTarget<DepthFormat> =
            gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}


impl Vertex {
    pub fn new(p: [f32; 3], t: [f32; 2], n: [f32; 3], ta: [f32; 3], c: [f32; 3]) -> Vertex {
        Vertex {
            pos: [p[0], p[1] , p[2]],
            tex_coord: [t[0], t[1]],
            normal: [n[0], n[1], n[2]],
            tangent: [ta[0], ta[1], ta[2]],
            color: [c[0], c[1], c[2]],
        }
    }
}

#[derive(Clone, Copy)]
pub enum MaterialType {
    OPAQUE,
    MASKED,
}


pub struct Object<R: gfx::Resources> {
    pub pso: gfx::PipelineState<R, my_pipe::Meta>,
    pub data: my_pipe::Data<R>,
    pub slices: gfx::Slice<R>,
    //3D Parameters
    pub world_location: Vector3<f32>,
    pub world_rotation: Vector3<f32>,
    pub world_scale: Vector3<f32>,
    //Material
    pub material: e_material::Material,
    pub material_type: MaterialType,

    pub is_active: bool,
}


impl<R: gfx::Resources> Object <R> {

    pub fn new<F>(  mut factory: &mut F,
                    main_color: &mut gfx::handle::RenderTargetView<R, ColorFormat>,
                    main_depth: &mut gfx::handle::DepthStencilView<R, DepthFormat>,
                    vertex_data: Vec<Vertex>, index_data: Vec<u32>,
                    material: &mut e_material::Material,
                    material_type: MaterialType,
                    light_manager: &e_lights_manager::LightManager) -> Self
    where F: gfx::Factory<R>,
    {
        let w_loc = Vector3::new(0.0, 0.0, 0.0);
        let w_rot: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
        let w_sca = Vector3::new(1.0, 1.0, 1.0);

        let i_material = material.clone();


        //load default shader
        let mut pso = factory.create_pipeline_simple(
            include_bytes!("shader/myshader_150.vs"),
            include_bytes!("shader/myshader_150.fs"),
            my_pipe::new()
        ).unwrap();

        //load shader Masked if needed
        match material_type {
            MaterialType::MASKED => pso = factory.create_pipeline_simple(
                                                include_bytes!("shader/masked.vs"),
                                                include_bytes!("shader/masked.fs"),
                                                my_pipe::new()
                                            ).unwrap(),
            MaterialType::OPAQUE => {},
        }




        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, index_data.as_slice());

        //let sampler = factory.create_sampler_linear();
        let sampler = factory.create_sampler(texture::SamplerInfo::new(texture::FilterMethod::Trilinear, texture::WrapMode::Tile));

        let diffuse_texture = gfx_load_texture::<F, R>(&mut factory, &i_material.diffuse);
        let specular_texture = gfx_load_texture::<F, R>(&mut factory, &i_material.specular);
        let normal_texture = gfx_load_texture::<F, R>(&mut factory, &i_material.normal);



        let mut data = my_pipe::Data {
            vbuf: vertex_buffer,
            locals: factory.create_constant_buffer(1),

            //Create light buffers according to the light settings

            dir_light: factory.create_buffer(light_manager.light_settings.max_dir_lights as usize, buffer::Role::Constant, memory::Usage::Dynamic, Bind::all()).unwrap(),
            spot_light: factory.create_buffer(light_manager.light_settings.max_spot_lights as usize, buffer::Role::Constant, memory::Usage::Dynamic, Bind::all()).unwrap(),
            point_light: factory.create_buffer(light_manager.light_settings.max_point_lights as usize, buffer::Role::Constant, memory::Usage::Dynamic, Bind::all()).unwrap(),

            //Create the light info for this object
            light_info: factory.create_constant_buffer(1),

            material: factory.create_constant_buffer(1),

            camera: factory.create_constant_buffer(1),

            //Create data with static textures for now
            diffuse_tex: (diffuse_texture,sampler.clone()),
            specular: (specular_texture,sampler.clone()),
            normal: (normal_texture,sampler.clone()),

            out_color: main_color.clone(),
            out_depth: main_depth.clone(),
        };


        Object {pso: pso,
                data: data,
                slices: slice,

                world_location: w_loc,
                world_scale: w_sca,
                world_rotation: w_rot,

                material_type: material_type,
                material: i_material,

                is_active: true,
            }
    }


//*************************************************************************************************
//Material
    pub fn get_material_instance(&mut self) -> &mut e_material::Material {
        &mut self.material
    }

//*************************************************************************************************
//3D space
    //Set world location
    pub fn set_world_location(&mut self, new_location: Vector3<f32>) {
        self.world_location = new_location;
    }

    //Add world location
    pub fn add_world_location(&mut self, add_ammount: Vector3<f32>) {
        self.world_location = Vector3::new(self.world_location.x + add_ammount.x,
                                            self.world_location.y + add_ammount.y,
                                            self.world_location.z + add_ammount.z);
    }

    pub fn get_world_location(&mut self) -> Vector3<f32>{
        self.world_location.clone()
    }


    pub fn set_world_rotation(&mut self, new: Vector3<f32>){
        self.world_rotation = new;
    }

    pub fn add_world_rotation(&mut self, new: Vector3<f32>){
        self.world_rotation = self.world_rotation + new;
    }


    pub fn set_world_scale(&mut self, new: Vector3<f32>){
        self.world_scale = new;
    }

    pub fn add_world_scale(&mut self, new: Vector3<f32>){
        self.world_scale = self.world_scale + new;
    }


//Render option
    pub fn set_active(&mut self, new: bool){
        self.is_active = new;
    }

    pub fn get_active(&mut self) -> bool {
        if self.is_active{
            true
        }else {
            false
        }
    }

}



//texture loader based on image crate
fn gfx_load_texture<F, R>(factory: &mut F, path: &String) -> gfx::handle::ShaderResourceView<R, [f32; 4]>
    where F: gfx::Factory<R>,
          R: gfx::Resources,
{
    use gfx::format::Rgba8;
    //have to flip v to make work with opengl
    let img = image::open(path.as_str()).unwrap().flipv().to_rgba();
    let (width, height) = img.dimensions();
    let kind = gfx::texture::Kind::D2(width as u16, height as u16, gfx::texture::AaMode::Single);
    let (_, view) = factory.create_texture_immutable_u8::<Rgba8>(kind, &[&img]).unwrap();
    view
}
