use crate::traits::GetPosition;

pub struct Camera {
    x: f32,
    y: f32,
    zoom: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32, zoom: f32) -> Self {
        Self {
            zoom: zoom,
            x: x,
            y: y,
        }
    }

    pub fn calculate(&self, x: f32, y: f32) -> (f32, f32) {
        ((x - self.x) * self.zoom, (y - self.y) * self.zoom)
    }

    pub fn follow<T: GetPosition>(&mut self, position: &T, factor: f32) {
        let (x, y) = position.get_position();
        self.x = self.x + (x - self.x) * factor;
        self.y = self.y + (y - self.y) * factor;
    }

    pub fn _x(&self) -> f32 {
        self.x
    }

    pub fn _y(&self) -> f32 {
        self.y
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }
}
