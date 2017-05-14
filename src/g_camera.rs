///Camera object used to create the "view" propertie of shaders
// Easly to be rewritten for custom cameras

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
}


impl Camera {
    pub fn new() -> Self {
        //camera General
        let cameraPos = Vector3::new(0.0, 0.0, 0.0);
        let cameraFront = Vector3::new(0.0, 0.0, -1.0);
        let cameraUp = Vector3::new(0.0, 1.0, 0.0);
        //Camera Rotation
        let yaw: f32 = 0.0;
        let pitch: f32 = 0.0;

        Camera {cameraPos: cameraPos, cameraFront: cameraFront, cameraUp: cameraUp, yaw: yaw, pitch: pitch,}
    }

    pub fn calc_view(&mut self, input_handler: &e_input::InputSystem, time_handler: &mut e_time::Time){

        let delta_time: f32 = time_handler.delta_time();


        //Corrected Camera Speed
        let camera_speed = 10.0 * delta_time;
        //println!("{:?}", delta_time);

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
    pub fn return_view_matrix(&self) -> [[f32; 4]; 4] {

        let tmp_target = self.cameraPos - self.cameraFront;
        let view =  Matrix4::look_at(
                    Point3::new(self.cameraPos.x, self.cameraPos.y, self.cameraPos.z),
                    Point3::new(tmp_target.x, tmp_target.y, tmp_target.z),
                    Vector3::new(self.cameraUp.x, self.cameraUp.y, self.cameraUp.z),
                ).into();
        view
    }
    pub fn get_direction(&self) -> Vector3<f32> {
        self.cameraFront
    }

    pub fn get_position(&self) -> Vector3<f32> {
        self.cameraPos
    }
}




//Helper function for calculating the view
fn to_radians(degree: f32) -> f32 {
    degree * (std::f64::consts::PI / 180.0) as f32
}
