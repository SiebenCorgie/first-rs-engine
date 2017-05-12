use glutin;

//keys struc (not finished for all keys) contains all pressed keys as well
//as mouse movement
pub struct InputMap {
    pub A: bool,
    pub W: bool,
    pub D: bool,
    pub S: bool,
    pub E: bool,
    pub Q: bool,
    pub C: bool,
    pub M: bool,
    pub CTRL_L: bool,
    pub SHIFT_L: bool,

    pub Arrow_Left: bool,
    pub Arrow_Right: bool,
    pub Arrow_Up: bool,
    pub Arrow_Down: bool,

    pub Mouse_x: i32,
    pub Mouse_y: i32,
    pub Delta_x: i32,
    pub Delta_y: i32,


}
//The Input system struct containing a keys struct with all, currently pressed keys
pub struct InputSystem {
    pub keys: InputMap,
}


impl InputSystem {
    //Creates a new input handler, no keys pressed
    pub fn new() -> Self {



        //Not optimal, better would be a bool map
        let mut keys: InputMap = InputMap{  A: false, W: false, S: false, D: false,
                                            E: false, Q: false, C: false,
                                            M: false,
                                            CTRL_L: false, SHIFT_L: false,
                                            Arrow_Left: false, Arrow_Right: false, Arrow_Up: false,
                                            Arrow_Down: false,
                                            Mouse_x: 0, Mouse_y: 0, Delta_x: 0, Delta_y: 0,
                                        };
        InputSystem {keys: keys}
    }

    //Checks all key and mouse states and puts them in "keys"
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
                                Some(glutin::VirtualKeyCode::M) => self.keys.M = true,

                                Some(glutin::VirtualKeyCode::Left) => self.keys.Arrow_Left = true,
                                Some(glutin::VirtualKeyCode::Right) => self.keys.Arrow_Right = true,
                                Some(glutin::VirtualKeyCode::Up) => self.keys.Arrow_Up = true,
                                Some(glutin::VirtualKeyCode::Down) => self.keys.Arrow_Down = true,

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
                                Some(glutin::VirtualKeyCode::M) => self.keys.M = false,

                                Some(glutin::VirtualKeyCode::Left) => self.keys.Arrow_Left = false,
                                Some(glutin::VirtualKeyCode::Right) => self.keys.Arrow_Right = false,
                                Some(glutin::VirtualKeyCode::Up) => self.keys.Arrow_Up = false,
                                Some(glutin::VirtualKeyCode::Down) => self.keys.Arrow_Down = false,

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
                                                                self.keys.Mouse_y = y_dir;
                                                            },

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

        //println!("X: {} ; Y: {}", self.keys.Delta_x, self.keys.Delta_y);

        //Arrow capturing
        let mut win_pos_x = 0 as i32;
        let mut win_pos_y = 0 as i32;

        let mut win_size_x = 0 as i32;
        let mut win_size_y = 0 as i32;

        let win_pos = window.get_position();
        match win_pos {
            Some((x,y)) => {    win_pos_x = x as i32;
                                win_pos_y = y as i32;},
            _ => {},
        }

        let win_size = window.get_inner_size();
        match win_size {
            Some((x, y)) => {   win_size_x = x as i32;
                                win_size_y = y as i32;},
            _ => {},
        }

        //if C is pressed make it possible to escape the window
        //Otherwise the curser always gets captured
        if self.keys.C == false {
            window.set_cursor_state(glutin::CursorState::Hide);

            let change = window.set_cursor_position((win_pos_x + (win_size_x / 2)), (win_pos_y + (win_size_y / 2)) as i32 );
        }else {
            window.set_cursor_state(glutin::CursorState::Normal);

        }

        //if everything worked return false... shouldn't close
        false
    }
}
