#[derive(Debug, Copy, Clone)]
pub struct Vector2d {
    x: f32,
    y: f32,
}

impl Vector2d {
    pub fn new(x: f32, y: f32) -> Vector2d {
        Vector2d {
            x: x,
            y: y,
        }
    }

    pub fn get_pos(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vector3d {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3d {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3d {
        Vector3d {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn get_pos(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }
}
