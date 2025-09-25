use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::world::{Tile, World};
use crate::camera::Camera;

const TILE_SIZE: f32 = 1.0;

pub struct Renderer {
    canvas: Canvas<Window>,
    width: u32,
    height: u32,
}

impl Renderer {
    pub fn new(video: &sdl2::VideoSubsystem, width: u32, height: u32) -> Result<Self, String> {
        let window = video.window("rust-sdl2 game", width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(Self { canvas, width, height })
    }

    pub fn render(&mut self, world: &World, camera: &Camera) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();

        for (y, row) in world.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let color = match tile {
                    Tile::Wall => Color::RGB(0, 0, 0),
                    Tile::Floor => Color::RGB(255, 255, 255),
                };
                self.canvas.set_draw_color(color);
                let (tile_x, tile_y) = camera.calculate((x as f32) * TILE_SIZE, (y as f32) * TILE_SIZE);
                let rect = Rect::new(
                    (tile_x + (self.width as f32) * 0.5) as i32,
                    (tile_y + (self.height as f32) * 0.5) as i32,
                (TILE_SIZE * camera.zoom()) as u32,
                (TILE_SIZE * camera.zoom()) as u32,
                );
                self.canvas.fill_rect(rect)?;
            }
        }

        self.canvas.present();
        Ok(())
    }
}
