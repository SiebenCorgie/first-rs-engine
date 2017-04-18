use glutin;


pub struct InputMap {
    pub A: bool,
    pub W: bool,
    pub D: bool,
    pub S: bool,
    pub E: bool,
    pub Q: bool,
    pub C: bool,
    pub CTRL_L: bool,
    pub SHIFT_L: bool,

    pub Mouse_x: i32,
    pub Mouse_y: i32,
    pub Delta_x: i32,
    pub Delta_y: i32,
}

pub struct InputSystem {
    pub keys: InputMap,
}


impl InputSystem {
    pub fn new() -> Self {
        //Not optimal, better would be a bool map
        let mut keys: InputMap = InputMap{  A: false, W: false, S: false, D: false,
                                            E: false, Q: false, C: false,
                                            CTRL_L: false, SHIFT_L: false,
                                            Mouse_x: 0, Mouse_y: 0, Delta_x: 0, Delta_y: 0};
        InputSystem {keys: keys}
    }

    pub fn process_events(&mut self, window: &glutin::Window) -> bool {
        //Processes Input and stores it in the keymap struct
        //returns true if should close
        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(State, _, Key) => {
                    match State {
                        glutin::ElementState::Pressed => {
                            match Key {
                                Some(glutin::VirtualKeyCode::Escape) => return true,
                                Some(glutin::VirtualKeyCode::W) => self.keys.W = true,
                                Some(glutin::VirtualKeyCode::A) => self.keys.A = true,
                                Some(glutin::VirtualKeyCode::S) => self.keys.S = true,
                                Some(glutin::VirtualKeyCode::D) => self.keys.D = true,
                                Some(glutin::VirtualKeyCode::E) => self.keys.E = true,
                                Some(glutin::VirtualKeyCode::Q) => self.keys.Q = true,
                                Some(glutin::VirtualKeyCode::C) => self.keys.C = true,
                                Some(glutin::VirtualKeyCode::LControl) => self.keys.CTRL_L = true,
                                Some(glutin::VirtualKeyCode::LShift) => self.keys.SHIFT_L = true,

                                _ => {},
                            }
                        }
                        glutin::ElementState::Released => {
                            match Key {
                                Some(glutin::VirtualKeyCode::W) => self.keys.W = false,
                                Some(glutin::VirtualKeyCode::A) => self.keys.A = false,
                                Some(glutin::VirtualKeyCode::S) => self.keys.S = false,
                                Some(glutin::VirtualKeyCode::D) => self.keys.D = false,
                                Some(glutin::VirtualKeyCode::E) => self.keys.E = false,
                                Some(glutin::VirtualKeyCode::Q) => self.keys.Q = false,
                                Some(glutin::VirtualKeyCode::C) => self.keys.C = false,
                                Some(glutin::VirtualKeyCode::LControl) => self.keys.CTRL_L = false,
                                Some(glutin::VirtualKeyCode::LShift) => self.keys.SHIFT_L = false,
                                _ => {},
                            }
                        },
                    }
                },

                //Mouse input
                glutin::Event::MouseMoved(x_dir, y_dir) => {    self.keys.Delta_x = self.keys.Mouse_x - x_dir;
                                                                self.keys.Delta_y = self.keys.Mouse_y - y_dir;
                                                                self.keys.Mouse_x = x_dir;
                                                                self.keys.Mouse_y = y_dir;},

                glutin::Event::Closed => return true,
                /*
                glutin::Event::Resized(_width, _height) => {

                    for i in 0..model_manager.len() {
                                gfx_window_glutin::update_views(&window, &mut model_manager[i].data.out_color, &mut main_depth);
                            }
                        }
                */
                _ => { // Reset mouse movement
                        self.keys.Delta_x = 0;
                        self.keys.Delta_y = 0;},

            }
        }

        //if everything worked return false... shouldn't close
        false
    }
}
