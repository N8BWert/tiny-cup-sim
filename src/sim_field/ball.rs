pub struct Ball {
    pub radius: u8,
    pub position: (f32, f32),
    pub velocity: (f32, f32),
}

impl Ball {
    pub fn new(radius: u8, position: (f32, f32)) -> Self {
        Self {
            radius,
            position,
            velocity: (0f32, 0f32)
        }
    }

    pub fn new_at_origin(radius: u8) -> Self {
        Self {
            radius,
            position: (0f32, 0f32),
            velocity: (0f32, 0f32),
        }
    }
}