pub struct Ball {
    position: [f32; 2],
    linear_velocity: [f32; 2],
}

impl Ball {
    pub const fn new(position: [f32; 2]) -> Self {
        Self {
            position,
            linear_velocity: [0f32; 2],
        }
    }

    pub fn ball_state(&self) -> BallState {
        BallState {
            position: self.position
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BallState {
    pub position: [f32; 2],
}