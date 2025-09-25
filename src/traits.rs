use crate::input::InputSubsystem;

pub trait GetPosition {
    fn get_position(&self) -> (f32, f32);
}

pub trait HandleInput {
    fn handle_input(&mut self, input: &mut InputSubsystem);
}
