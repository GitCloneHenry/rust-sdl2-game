use sdl2::{event::Event, keyboard::Keycode};
use std::time::Duration;

use crate::camera::Camera;
use crate::input::InputSubsystem;
use crate::player::Player;
use crate::render::Renderer;
use crate::traits::HandleInput;
use crate::world::World;

pub fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video = sdl_context.video()?;

    let mut renderer = Renderer::new(&video, 800, 800)?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut input = InputSubsystem::new();

    let world = World::from("src/map.txt")?;
    let mut camera = Camera::new(5.0, 5.0, 160.0);
    let mut player = Player::new(5.0, 5.0, 0.4, 0.8);

    'running: loop {
        for event in event_pump.poll_iter() {
            input.handle_event(&event);

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        player.handle_input(&mut input);
        camera.follow(&player, 0.5);

        renderer.render(&world, &camera, &player)?;
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
