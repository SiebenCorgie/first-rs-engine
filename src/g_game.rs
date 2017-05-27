
use g_object;
use cgmath::*;
use gfx;
use time;

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
    bounds: Bounds
}

impl Game {
    pub fn new () -> Self{
        Game{motion: Motion{x: 10.0, y: 20.0},
            bounds: Bounds{x: 80.0, y: 80.0, mx: -80.0, my: -80.0}}
    }

    pub fn update<R>(&mut self, ball: &mut g_object::Object<R>, delta_time: f32) where R: gfx::Resources{

        let current_loc = ball.get_world_location();

        //Change motion when touching short end
        if current_loc.x > self.bounds.x || current_loc.x < self.bounds.mx{
            self.motion.x = -1.0 * self.motion.x;
            //self.motion.y = -1.0 * self.motion.y;
        }
        //Change only y for long ends

        if current_loc.z > self.bounds.y || current_loc.z < self.bounds.my{
            self.motion.y = -1.0 * self.motion.y;
        }


        ball.add_world_location(Vector3::new(self.motion.x * delta_time, 0.0, self.motion.y * delta_time));
    }
}
