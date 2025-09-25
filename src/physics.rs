use crate::player::Player;

pub struct AABB {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

impl AABB {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        AABB { x1, y1, x2, y2 }
    }

    pub fn compare_against(&self, aabb: AABB) -> bool {
        (self.x1 > aabb.x1 && self.y1 > aabb.y1 && self.x1 < aabb.x2 && self.y1 < aabb.y2) ||
        (self.x2 > aabb.x1 && self.y2 > aabb.y1 && self.x2 < aabb.x2 && self.y2 < aabb.y2)
    }
}

pub struct RigidBody {
    bounding_box: AABB,
    is_static: bool,
    dx: f32,
    dy: f32,
}

impl RigidBody {
    pub fn new(bounding_box: AABB, is_static: bool, dx: Option<f32>, dy: Option<f32>) -> Self {
        let dx = dx.map(|dx| dx as f32).unwrap_or(0.0);
        let dy = dy.map(|dy| dy as f32).unwrap_or(0.0);

        RigidBody {
            bounding_box: bounding_box,
            is_static: is_static,
            dx: dx,
            dy: dy
        }
    }
}

pub struct PhysicsSubsystem {
    gravity: f32
}

impl PhysicsSubsystem {
    pub fn new(gravity: f32) {
        PhysicsSubsystem {
            gravity
        }
    }
}
