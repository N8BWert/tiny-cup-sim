pub struct Robot {
    position: [f32; 2],
    rotation: f32,
    linear_velocity: [f32; 2],
    angular_velocity: f32,
}

impl Robot {
    pub const fn new(position: [f32; 2], rotation: f32) -> Self {
        Self {
            position,
            rotation,
            linear_velocity: [0f32; 2],
            angular_velocity: 0f32,
        }
    }

    pub fn robot_state(&self) -> RobotState {
        RobotState {
            position: self.position,
            rotation: self.rotation,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RobotState {
    pub position: [f32; 2],
    pub rotation: f32,
}