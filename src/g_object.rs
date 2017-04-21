use image;
use cgmath;
#[macro_use]
use gfx;
use gfx_window_glutin;

use gfx::traits::FactoryExt;
use gfx::{Bundle, texture, Device};

use t_obj_importer;

use cgmath::{Point3, Vector3};
use cgmath::{Transform, AffineMatrix3};

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

    //Cube Pipeline
    pipeline my_pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        //transform: gfx::Global<[[f32; 4]; 4]> = "u_Transform",
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        color: gfx::TextureSampler<[f32; 4]> = "t_Color",
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
    pub name: String,
    pub pso: gfx::PipelineState<R, my_pipe::Meta>,
    pub data: my_pipe::Data<R>,
    pub slices: gfx::Slice<R>,
    //3D Parameters
    pub world_location: Vector3<f32>,
}


impl<R: gfx::Resources> Object <R> {

    pub fn new<F>(  factory: &mut F,
                    main_color: &mut gfx::handle::RenderTargetView<R, ColorFormat>,
                    main_depth: &mut gfx::handle::DepthStencilView<R, DepthFormat>,
                    vertex_data: Vec<Vertex>, index_data: Vec<u32>) -> Self
    where F: gfx::Factory<R>,
    {
        let w_loc = Vector3::new(0.0, 0.0, 0.0);

        //Create Triangle
        let pso = factory.create_pipeline_simple(
            include_bytes!("shader/myshader_150.vs"),
            include_bytes!("shader/myshader_150.fs"),
            my_pipe::new()
        ).unwrap();
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, index_data.as_slice());

        let sampler = factory.create_sampler_linear();

        let view = {
            use gfx::format::Rgba8;
            //need to flip h, to make work with gfx-rs/opengl
            let img = image::open("/share/Photogrammetry/_FinalModels/Journey/Small_Monuments/Buddha_White/Buddha_Diff.png").unwrap().flipv().to_rgba();
            let (width, height) = img.dimensions();
            let kind = gfx::texture::Kind::D2(width as u16, height as u16, gfx::texture::AaMode::Single);
            let (_, view) = factory.create_texture_immutable_u8::<Rgba8>(kind, &[&img]).unwrap();
            view
        };


        //let texture = gfx_load_texture(&mut factory);
        let texture = view;
        let proj = cgmath::perspective(cgmath::deg(45.0f32), (1024.0/768.0), 1.0, 10.0);


        let mut data = my_pipe::Data {
            vbuf: vertex_buffer,
            //transform: (proj * default_view([1.5f32, -5.0, 3.0], [0f32, 0.0, 0.0] ).mat).into(),
            locals: factory.create_constant_buffer(1),
            color: (texture, sampler),
            out_color: main_color.clone(),
            out_depth: main_depth.clone(),
        };


        Object {name: String::from("Teddy"),
                pso: pso,
                data: data,
                slices: slice,
                world_location: w_loc,
                }
    }

    pub fn set_world_location(&mut self, new_location: Vector3<f32>) {
        self.world_location = new_location;
    }

    pub fn import_mesh(&mut self, path: &str){

    }

}
