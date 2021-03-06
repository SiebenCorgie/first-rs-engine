///Camera object used to create the "view" propertie of shaders
// Easly to be rewritten for custom cameras
use e_engine_settings;

//Camera trait, use this to implement any type of camera
pub trait CameraTyp {
    //Creates a default camera
    fn new() -> Self;
    //Calculates / Update the view
    fn calc_view(&mut self, input_handler: &e_input::InputSystem, time_handler: &mut e_time::Time);
    //Returns the view matrix if needed
    fn return_view_matrix(&self) -> [[f32; 4]; 4];
    //Returns the current direction of the camera
    fn get_direction(&self) -> Vector3<f32>;
    //Set current direction
    fn set_direction(&mut self, new_direction: Vector3<f32>);
    //Returns Position
    fn get_position(&self) -> Vector3<f32>;
    //Set current position
    fn set_position(&mut self, new_pos: Vector3<f32>);
    //Sets Fov on this camera
    fn set_fov(&mut self, new_fov: f32);
    //Sets the far, and near planes of the frustum
    fn set_frustum_planes(&mut self, near: f32, far: f32);
    //Returns the perspective matrix based on the window settings
    fn get_perspective(&self, engine_settings: &e_engine_settings::EngineSettings) -> [[f32;4]; 4];
}


use cgmath::*;
use e_input;
use e_time;
use glutin::Window;
use std;

pub struct Camera {
    //camera General
    pub cameraPos: Vector3<f32>,
    pub cameraFront: Vector3<f32>,
    pub cameraUp: Vector3<f32>,
    //Camera Rotation
    yaw: f32,
    pitch: f32,

    //Setting
    fov: f32,
    near_plane: f32,
    far_plane: f32,
}


impl CameraTyp for Camera{
    fn new() -> Self {
        //camera General
        let cameraPos = Vector3::new(0.0, 0.0, 0.0);
        let cameraFront = Vector3::new(0.0, 0.0, -1.0);
        let cameraUp = Vector3::new(0.0, 1.0, 0.0);
        //Camera Rotation
        let yaw: f32 = 0.0;
        let pitch: f32 = 0.0;

        let fov = 45.0;
        let near_plane = 0.1;
        let far_plane = 100.0;

        Camera {cameraPos: cameraPos, cameraFront: cameraFront,
            cameraUp: cameraUp, yaw: yaw, pitch: pitch,
            fov: fov, near_plane: near_plane, far_plane: far_plane}
    }

    fn calc_view(&mut self, input_handler: &e_input::InputSystem, time_handler: &mut e_time::Time){

        let delta_time: f32 = time_handler.delta_time();


        //Corrected Camera Speed
        let camera_speed = 50.0 * delta_time;

        //Input processing
        {
            if input_handler.keys.A == true {
                self.cameraPos = self.cameraPos + (self.cameraFront.cross(self.cameraUp).normalize()) * camera_speed;
            }
            if input_handler.keys.W == true {
                self.cameraPos = self.cameraPos - self.cameraFront * camera_speed;
            }
            if input_handler.keys.S == true {
                self.cameraPos = self.cameraPos + self.cameraFront * camera_speed;
            }
            if input_handler.keys.D == true {
                self.cameraPos = self.cameraPos - (self.cameraFront.cross(self.cameraUp).normalize()) * camera_speed;
            }
            if (input_handler.keys.CTRL_L == true) | (input_handler.keys.Q == true) {
                self.cameraPos = self.cameraPos - Vector3::new(0.0, camera_speed, 0.0);
            }
            if (input_handler.keys.SHIFT_L == true) | (input_handler.keys.E == true) {
                self.cameraPos = self.cameraPos + Vector3::new(0.0, camera_speed, 0.0);
            }


        }

        let sensitivity = 10.0;

        //Fixed camera gittering by slowing down so one integer delta = movement of
        // delta * sensitvity * time_delta * slowdown (virtual speed up)
        let virtual_speedup = 0.25;
        let x_offset: f32 = input_handler.keys.Delta_x as f32 * sensitivity * time_handler.delta_time() * virtual_speedup;
        let y_offset: f32 = input_handler.keys.Delta_y as f32 * sensitivity * time_handler.delta_time() * virtual_speedup;

        self.yaw += x_offset;
        self.pitch += y_offset;

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        let mut front = Vector3::new(0.0, 0.0, 0.0);
        front.x = to_radians(self.yaw).cos() * to_radians(self.pitch).cos();
        front.y = to_radians(self.pitch).sin();
        front.z =  to_radians(self.yaw).sin() * to_radians(self.pitch).cos();
        self.cameraFront = front.normalize();

    }

    //Return view matrix as [[f32; 4]; 4]
    fn return_view_matrix(&self) -> [[f32; 4]; 4] {

        let tmp_target = self.cameraPos - self.cameraFront;
        let view =  Matrix4::look_at(
                    Point3::new(self.cameraPos.x, self.cameraPos.y, self.cameraPos.z),
                    Point3::new(tmp_target.x, tmp_target.y, tmp_target.z),
                    Vector3::new(self.cameraUp.x, self.cameraUp.y, self.cameraUp.z),
                ).into();
        view
    }
    fn get_direction(&self) -> Vector3<f32> {
        self.cameraFront
    }

    fn set_direction(&mut self, new_direction: Vector3<f32>){
        self.cameraFront = new_direction.normalize();
    }

    fn get_position(&self) -> Vector3<f32> {
        self.cameraPos
    }
    fn set_position(&mut self, new_pos: Vector3<f32>){
        self.cameraPos = new_pos;
    }

    fn set_fov(&mut self, new_fov: f32){
        self.fov = new_fov;
    }

    fn set_frustum_planes(&mut self, near: f32, far: f32) {
        self.far_plane = far;
        self.near_plane = near;
    }

    //Calculates the perspective based on the engine and camera settings
    fn get_perspective(&self, engine_settings: &e_engine_settings::EngineSettings) -> [[f32;4]; 4]{

        perspective(Deg(self.fov),
        (engine_settings.width as f32 / engine_settings.height as f32),
        self.near_plane, self.far_plane).into()
    }
}




//Helper function for calculating the view
fn to_radians(degree: f32) -> f32 {
    degree * (std::f64::consts::PI / 180.0) as f32
}
