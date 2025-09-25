mod game;
mod world;
mod render;
mod camera;

fn main() -> Result<(), String> {
    game::run()
}
