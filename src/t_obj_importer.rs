
use std::path::Path;
use tobj;
use g_object;

pub struct Importer {}

impl Importer {
    pub fn new() -> Self {
        Importer {}
    }

    pub fn import_mesh(&self, path: &str)-> (Vec<Vec<g_object::Vertex>>, Vec<Vec<u32>>, Vec<String>){

        let mut object_pile: Vec<Vec<g_object::Vertex>> = Vec::new();
        let mut index_pile: Vec<Vec<u32>> = Vec::new();
        let mut object_name: Vec<String> = Vec::new();



        //load
        let object = tobj::load_obj(&Path::new(path));
        assert!(object.is_ok());
        let (mut models, mut materials) = object.unwrap();

        //println!("# of models: {}", models.len());
        //println!("# of materials: {}", materials.len());

        //Push to object_vertex
        for (i, m) in models.iter().enumerate() {

            let mut object_vertex: Vec<g_object::Vertex> = Vec::new();
            let mut object_indices: Vec<u32> = Vec::new();

            let mesh = &m.mesh;
        	//println!("model[{}].name = \'{}\'", i, m.name);
            //Give to name pile
            object_name.push(String::from(m.name.clone()));

            //println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);

        	//println!("Size of model[{}].indices: {}", i, mesh.indices.len());
        	for f in 0..mesh.indices.len() / 3 {
        		//println!("    idx[{}] = {}, {}, {}.", f, mesh.indices[3 * f],
        		//	mesh.indices[3 * f + 1], mesh.indices[3 * f + 2]);

                //Write to object_indices
                object_indices.push(mesh.indices[3 * f]);
                object_indices.push(mesh.indices[3 * f + 1]);
                object_indices.push(mesh.indices[3 * f + 2]);

        	}

        	// Normals and texture coordinates are also loaded, but not printed in this example
        	println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);
        	assert!(mesh.positions.len() % 3 == 0);

            //Create prototype_Mesh_Positions
        	for v in 0..mesh.positions.len() / 3 {
        		println!("    v[{}] = ({}, {}, {})", v, mesh.positions[3 * v],
        			mesh.positions[3 * v + 1], mesh.positions[3 * v + 2]);

                println!("    uv[{}] = (u: {}, v: {})", v, mesh.texcoords[v * 2],
            		mesh.texcoords[v * 2 + 1],);

                if (!mesh.normals.is_empty()) && (!mesh.texcoords.is_empty()) {
                    //Write Vertex_Struct for all vertex of this object
                    object_vertex.push(g_object::Vertex::new(
                        [mesh.positions[3 * v], mesh.positions[3 * v + 1],  mesh.positions[3 * v + 2]], //Location
                        [mesh.texcoords[2 * v], mesh.texcoords[2 * v + 1]], //UV
                        [mesh.normals[3 * v], mesh.normals[3 * v + 1], mesh.normals[3 * v + 2]], //Normal
                        [1.0, 1.0, 1.0],//Color (vertex) //currently not supported
                        [1.0, 1.0, 1.0],//Dead Tangent
                        ));


                }
                else {
                    //Write Vertex_Struct for all vertex of this object (only position)
                    println!("ERROR: NO TEXCOORDS OR NORMALS FOUND");
                    object_vertex.push(g_object::Vertex::new(
                        [mesh.positions[3 * v], mesh.positions[3 * v + 1],  mesh.positions[3 * v + 2]], //Location
                        [1.0, 0.0], //UV
                        [0.0, 0.0, 0.0], //Normal
                        [1.0, 1.0, 1.0],//Color (vertex) //currently not supported
                        [1.0, 1.0, 1.0],//Dead Tangent
                        ));
                }
        	}
            //Push Vertexes of this object to the pile
            object_pile.push(object_vertex);
            //Push indices to the indice Pile
            index_pile.push(object_indices);
        }

        /*
        //Maybe later :)
        for (i, m) in materials.iter().enumerate() {
        	println!("material[{}].name = \'{}\'", i, m.name);
        	println!("    material.Ka = ({}, {}, {})", m.ambient[0], m.ambient[1],
        		m.ambient[2]);
        	println!("    material.Kd = ({}, {}, {})", m.diffuse[0], m.diffuse[1],
        		m.diffuse[2]);
        	println!("    material.Ks = ({}, {}, {})", m.specular[0], m.specular[1],
        		m.specular[2]);
        	println!("    material.Ns = {}", m.shininess);
        	println!("    material.d = {}", m.dissolve);
        	println!("    material.map_Ka = {}", m.ambient_texture);
        	println!("    material.map_Kd = {}", m.diffuse_texture);
        	println!("    material.map_Ks = {}", m.specular_texture);
        	println!("    material.map_Ns = {}", m.normal_texture);
        	println!("    material.map_d = {}", m.dissolve_texture);
        	for (k, v) in &m.unknown_param {
        		println!("    material.{} = {}", k, v);
        	}
        }
        */
        (object_pile, index_pile, object_name)
    }
}
