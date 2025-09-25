mod camera;
mod game;
mod input;
mod player;
mod render;
mod traits;
mod world;

fn main() -> Result<(), String> {
    game::run()
}
