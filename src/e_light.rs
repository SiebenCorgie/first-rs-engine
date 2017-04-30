use cgmath::*;



//Structs are exactly the same like the ones
//in gfx_define!
//They'll form the base for any light combination
pub struct Light_Directional {
    pub d_lightDirection: [f32; 4] ,
    pub d_lightColor: [f32; 4] ,
    pub d_lightStrength: f32 ,
}
pub struct Light_Spot {
    pub s_lightPos: [f32; 4] ,
    pub s_lightDirection: [f32; 4] ,
    pub s_lightColor: [f32; 4] ,
    pub s_cutOff: f32 ,

}
pub struct Light_Point {
    pub p_lightPos: [f32; 4] ,
    pub p_lightColor: [f32; 4] ,
    pub p_constant: f32 ,
    pub p_linear: f32 ,
    pub p_quadratic: f32 ,
    pub p_lightStrength: f32 ,
}


//Directinoal light
impl Light_Directional {
    //returns a directional light
    pub fn new(direction: Vector3<f32>, color: Vector3<f32>, strength: f32) -> Self {
        Light_Directional{  d_lightDirection: direction.extend(1.0).into(),
                            d_lightColor: color.extend(1.0).into(),
                            d_lightStrength: strength
                        }
    }
    //Set_er
    pub fn set_direction(&mut self, new_direction: Vector3<f32>){
        self.d_lightDirection = new_direction.extend(1.0).into();
    }

    pub fn set_color(&mut self, new_color: Vector3<f32>){
        self.d_lightColor = new_color.extend(1.0).into();
    }

    pub fn set_strength(&mut self, new_strength: f32){
        self.d_lightStrength = new_strength;
    }
}

//Point light
impl Light_Point{
    pub fn new( position: Vector3<f32>,
                color: Vector3<f32>, constant: f32,
                linear: f32, quadratic: f32, strength: f32) -> Self {
        //Still have to store a Vec4 because some offset problems
        //When passing to glsl in the pipline
        Light_Point{p_lightPos: position.extend(1.0).into(),
                    p_lightColor: color.extend(1.0).into(),
                    p_constant: constant ,
                    p_linear: linear ,
                    p_quadratic: quadratic ,
                    p_lightStrength: strength}
    }
    pub fn set_position(&mut self, new_pos: Vector3<f32>){
        self.p_lightPos = new_pos.extend(1.0).into();
    }

    pub fn set_color(&mut self, new_color: Vector3<f32>){
        self.p_lightColor = new_color.extend(1.0).into();
    }

    pub fn set_constant(&mut self, new_constant: f32){
        self.p_constant = new_constant;
    }

    pub fn set_linear(&mut self, new_linear: f32){
        self.p_linear = new_linear;
    }

    pub fn set_quadratic(&mut self, new_quadratic: f32){
        self.p_quadratic = new_quadratic;
    }

    pub fn set_strength(&mut self, new_strength: f32){
        self.p_lightStrength = new_strength;
    }

}

//Spot light
impl Light_Spot {
    pub fn new( position: Vector3<f32>, direction: Vector3<f32>,
                color: Vector3<f32>, cut_off: f32) -> Self {
        //Still have to store a Vec4 because some offset problems
        //When passing to glsl in the pipline
        Light_Spot{ s_lightPos: position.extend(1.0).into(),
                    s_lightDirection: direction.extend(1.0).into(),
                    s_lightColor: color.extend(1.0).into(),
                    s_cutOff: cut_off}
    }
    pub fn set_position(&mut self, new_pos: Vector3<f32>){
        self.s_lightPos = new_pos.extend(1.0).into();
    }

    pub fn set_direction(&mut self, new_direction: Vector3<f32>){
        self.s_lightDirection = new_direction.extend(1.0).into();
    }

    pub fn set_color(&mut self, new_color: Vector3<f32>){
        self.s_lightColor = new_color.extend(1.0).into();
    }

    pub fn set_cut_off(&mut self, new_cut_off: f32){
        self.s_cutOff = new_cut_off;
    }

}
