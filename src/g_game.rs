
use g_object;
use cgmath::*;
use gfx;
use time;


pub struct Block {
    width: f32,
    length: f32,
}

pub struct Motion {
    x: f32,
    y: f32,
}

pub struct Bounds {
    x: f32,
    y: f32,
    mx: f32,
    my: f32,
}

pub struct Game {
    motion: Motion,
    bounds: Bounds,
    block_1: Block,
    block_2: Block,
    block_3: Block,

    block_speed: f32,
}

impl Game {
    pub fn new () -> Self{
        Game{motion: Motion{x: 40.0, y: 20.0},
            bounds: Bounds{x: 80.0, y: 80.0, mx: -80.0, my: -80.0},
            block_1: Block{width: 10.0, length: 80.0},
            block_2: Block{width: 10.0, length: 80.0},
            block_3: Block{width: 10.0, length: 80.0},
            block_speed: 25.0,
        }
    }

    pub fn update<R>(&mut self, ball: &mut g_object::Object<R>, delta_time: f32,
        block_r: Vector3<f32>, block_l: Vector3<f32>, ) where R: gfx::Resources{

        let current_loc = ball.get_world_location();

        let bounce_offset = 5.0;

        //Change motion when touching short end
        if current_loc.x > self.bounds.x - bounce_offset{
            if current_loc.z > block_r.z + (self.block_1.length / 2.0)
                || current_loc.z < block_r.z - (self.block_1.length / 2.0)
            {
                //Score for left
                println!("Score for left", );
                ball.set_world_location(Vector3::new(50.0, 0.0, 0.0));
                self.motion.x = -40.0;
            }else{
                //bounce from right side
                self.motion.x = -1.0 * self.motion.x;
            }
        }

        if current_loc.x < self.bounds.mx + bounce_offset{
            if current_loc.z > block_l.z + (self.block_2.length / 2.0)
                || current_loc.z < block_l.z - (self.block_2.length / 2.0)
            {
                //Score for left
                println!("Score for right", );
                ball.set_world_location(Vector3::new(-50.0, 0.0, 0.0));
                self.motion.x = 40.0;
            }else{
                //bounce from right side
                self.motion.x = -1.0 * self.motion.x;
            }
        }


        //touching long end
        if current_loc.z > self.bounds.y || current_loc.z < self.bounds.my{
            self.motion.y = -1.0 * self.motion.y;
        }


        ball.add_world_location(Vector3::new(self.motion.x * delta_time, 0.0, self.motion.y * delta_time));
    }
    //Move first
    pub fn move_1<R>(&mut self, object: &mut g_object::Object<R>, up: bool, delta_time: f32) where R: gfx::Resources{

        match up {
            true => {
                if object.get_world_location().z < (self.bounds.y - (self.block_1.length / 2.0)) {
                    object.add_world_location(Vector3::new(0.0, 0.0, self.block_speed * delta_time));
                }
            },
            false => {
                if object.get_world_location().z > (self.bounds.my + (self.block_1.length / 2.0)) {
                    object.add_world_location(Vector3::new(0.0, 0.0, -1.0 * self.block_speed * delta_time));
                }
            },
        }

    }

    //Move second
    pub fn move_2<R>(&mut self, object: &mut g_object::Object<R>, up: bool, delta_time: f32) where R: gfx::Resources{

        match up {
            true => {
                if object.get_world_location().z < (self.bounds.y - (self.block_2.length / 2.0)) {
                    object.add_world_location(Vector3::new(0.0, 0.0, self.block_speed * delta_time));
                }
            },
            false => {
                if object.get_world_location().z > (self.bounds.my + (self.block_2.length / 2.0)) {
                    object.add_world_location(Vector3::new(0.0, 0.0, -1.0 * self.block_speed * delta_time));
                }
            },
        }

    }
}
