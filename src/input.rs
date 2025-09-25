use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;

pub struct InputSubsystem {
    held: HashSet<Keycode>,
}

impl InputSubsystem {
    pub fn new() -> Self {
        Self {
            held: HashSet::new(),
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(k),
                repeat: false,
                ..
            } => {
                self.held.insert(*k);
            }
            Event::KeyUp {
                keycode: Some(k), ..
            } => {
                self.held.remove(&k);
            }
            _ => {}
        }
    }

    pub fn is_pressed(&mut self, key: Keycode) -> bool {
        self.held.contains(&key)
    }
}
