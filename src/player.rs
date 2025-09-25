use sdl2::keyboard::Keycode;

use crate::input::InputSubsystem;
use crate::traits::{GetPosition, HandleInput};

pub struct Player {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl GetPosition for Player {
    fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl HandleInput for Player {
    fn handle_input(&mut self, input: &mut InputSubsystem) {
        if input.is_pressed(Keycode::W) {
            self.y -= 0.1;
        }
        if input.is_pressed(Keycode::A) {
            self.x -= 0.1;
        }
        if input.is_pressed(Keycode::S) {
            self.y += 0.1;
        }
        if input.is_pressed(Keycode::D) {
            self.x += 0.1;
        }
    }
}

impl Player {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }
}
