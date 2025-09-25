use sdl2::{event::Event, keyboard::Keycode};
use std::time::Duration;

use crate::camera::Camera;
use crate::world::World;
use crate::render::Renderer;

pub fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video = sdl_context.video()?;

    let mut renderer = Renderer::new(&video, 800, 800)?;
    let mut event_pump = sdl_context.event_pump()?;

    let world = World::from("src/map.txt")?;
    let camera = Camera::new(5.0, 5.0, 80.0);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } 
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        renderer.render(&world, &camera)?;
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
