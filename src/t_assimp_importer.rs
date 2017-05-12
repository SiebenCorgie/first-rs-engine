
use std::path::Path;

use assimp;
use g_object;
//A new assimp importer for more modesl and tangent sapce/ bitangent space import


pub struct Importer {}

impl Importer {
    pub fn new() -> Self {
        Importer{}
    }

    pub fn import_mesh(&self, path: &str)-> (Vec<Vec<g_object::Vertex>>, Vec<Vec<u32>>, Vec<String>){


        let mut object_pile: Vec<Vec<g_object::Vertex>> = Vec::new();
        let mut index_pile: Vec<Vec<u32>> = Vec::new();
        let mut object_name: Vec<String> = Vec::new();
        //load (assimp)
        let mut importer = assimp::Importer::new();


        importer.calc_tangent_space(|x| x.enable = true);
        //might need this importer.flip_uvs(true);
        importer.triangulate(true);
        importer.generate_normals(|x| x.enable = true);
        importer.pre_transform_vertices(|x| {
            x.enable = true;
            x.normalize = true;
        });

        //Import scene with all meshes
        let scene = importer.read_file(path.clone()).unwrap();

        for mesh in scene.mesh_iter() {

            //get name from path
            let name = Path::new(path).file_stem().unwrap().to_str().unwrap();

            object_name.push(String::from(name));
            println!("{:?}", (String::from(name)));


            let mut object_vertex: Vec<g_object::Vertex> = Vec::new();



            //Only import valid meshes

                //The vertex
                for index in 0..mesh.num_vertices()
                {
                    let mut pos: [f32; 3] = [0.0; 3];
                    let mut tex: [f32; 2] = [0.0; 2];
                    let mut norm: [f32; 3] = [0.0; 3];
                    let mut tang: [f32; 3] = [0.0; 3];
                    let mut col: [f32; 3] = [0.0; 3];

                    //Set position (has to have positions)
                    if true {
                        //println!("has Pos");
                        pos = mesh.get_vertex(index).unwrap().into();
                    }
                    //Has to have tex_coords
                    if true {
                        //println!("has texcoords");
                        let Vec3: [f32; 3] = mesh.get_texture_coord(0, index).unwrap().into();
                        tex = [Vec3[0], Vec3[1]];
                    }

                    if mesh.has_normals(){
                        //println!("has normals");
                        norm = mesh.get_normal(index).unwrap().into();
                        //println!("normal: {}, {}, {}", norm[0], norm[1], norm[2] );
                    }

                    if mesh.has_tangents_and_bitangents(){
                        //println!("has tangent");
                        tang = mesh.get_tangent(index).unwrap().into();
                        //println!("tangent: {}, {}, {}", tang[0], tang[1], tang[2] );
                    }

                    if mesh.has_vertex_colors(index as usize){
                        //println!("has color");
                        col = mesh.get_tangent(index).unwrap().into();
                    }

                    object_vertex.push(g_object::Vertex::new(pos, tex, norm, tang, col));

                }

                //The indice
                let mut object_indices: Vec<u32> = Vec::new();
                // Safe to assume all faces are triangles due to import options
                for face in mesh.face_iter() {
                    object_indices.push(face[0]);
                    object_indices.push(face[1]);
                    object_indices.push(face[2]);
                    //println!("Indices: {}, {}, {}", face[0], face[1], face[2]);
                }

                //Push both, vertex and indieces of this object to the piles
                object_pile.push(object_vertex);
                index_pile.push(object_indices);
            }

        (object_pile, index_pile, object_name)
    }
}
