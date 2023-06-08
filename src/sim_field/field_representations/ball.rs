pub struct Ball {
    pub radius: u8,
    pub position: (f64, f64),
    pub velocity: (f64, f64),
}

impl Ball {
    pub fn new(radius: u8, position: (f64, f64)) -> Self {
        Self {
            radius,
            position,
            velocity: (0f64, 0f64)
        }
    }

    pub fn new_at_origin(radius: u8) -> Self {
        Self {
            radius,
            position: (0f64, 0f64),
            velocity: (0f64, 0f64),
        }
    }

    pub fn add_velocity(&mut self, velocity: (f64, f64)) {
        self.velocity.0 += velocity.0;
        self.velocity.1 += velocity.1;
    }

    pub fn update_position(&mut self, delta: u128) {
        self.position.0 += self.velocity.0 * (delta as f64) * 0.001;
        self.position.1 += self.velocity.1 * (delta as f64) * 0.001;
    }
}