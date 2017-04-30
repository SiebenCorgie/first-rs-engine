use image;
use cgmath;
#[macro_use]
use gfx;
use gfx_window_glutin;

use gfx::traits::FactoryExt;
use gfx::*;

use t_obj_importer;
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
        _pad3: f32 = "_pad3",
        d_active: bool = "d_active",
    }
    constant Light_Spot {
        s_lightPos: [f32; 4] = "s_lightPos",
        s_lightDirection: [f32; 4] = "s_lightDirection",
        s_lightColor: [f32; 4] = "s_lightColor",
        s_cutOff: f32 = "s_cutOff",
        _pad1: f32 = "_pad1",
        _pad2: f32 = "_pad2",
        _pad3: f32 = "_pad3",
        s_active: bool = "s_active",
    }
    constant Light_Point {
        p_lightPos: [f32; 4] = "p_lightPos",
        p_lightColor: [f32; 4] = "p_lightColor",
        p_constant: f32 = "p_constant",
        p_linear: f32 = "p_linear",
        p_quadratic: f32 = "p_quadratic",
        p_lightStrength: f32 = "p_lightStrength",
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
        out_color: gfx::RenderTarget<ColorFormat> = "Target0",
        out_depth: gfx::DepthTarget<DepthFormat> =
            gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}


impl Vertex {
    pub fn new(p: [f32; 3], t: [f32; 2], n: [f32; 3], c: [f32; 3]) -> Vertex {
        Vertex {
            pos: [p[0], p[1] , p[2]],
            tex_coord: [t[0], t[1]],
            normal: [n[0], n[1], n[2]],
            color: [c[0], c[1], c[2]],
        }
    }
}


pub struct Object<R: gfx::Resources> {
    pub pso: gfx::PipelineState<R, my_pipe::Meta>,
    pub data: my_pipe::Data<R>,
    pub slices: gfx::Slice<R>,
    //3D Parameters
    pub world_location: Vector3<f32>,
    pub world_rotation: Matrix3<f32>,
    pub world_scale: Vector3<f32>,
    //Material
    pub material: e_material::Material,
}


impl<R: gfx::Resources> Object <R> {

    pub fn new<F>(  mut factory: &mut F,
                    main_color: &mut gfx::handle::RenderTargetView<R, ColorFormat>,
                    main_depth: &mut gfx::handle::DepthStencilView<R, DepthFormat>,
                    vertex_data: Vec<Vertex>, index_data: Vec<u32>,
                    material: &mut e_material::Material,
                    light_manager: &e_lights_manager::LightManager) -> Self
    where F: gfx::Factory<R>,
    {
        let w_loc = Vector3::new(0.0, 0.0, 0.0);
        let w_rot: Matrix3<f32> = Matrix3::from_value(0.0);
        let w_sca = Vector3::new(0.0, 0.0, 0.0);

        let i_material = material.clone();


        //Create Triangle
        let pso = factory.create_pipeline_simple(
            include_bytes!("shader/myshader_150.vs"),
            include_bytes!("shader/myshader_150.fs"),
            my_pipe::new()
        ).unwrap();
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, index_data.as_slice());

        let sampler = factory.create_sampler_linear();


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
                material: i_material,
                slices: slice,
                world_location: w_loc,
                world_scale: w_sca,
                world_rotation: w_rot,
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
