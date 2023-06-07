use crate::sim_field::team::Team;

pub struct Robot {
    pub team: Team,
    pub position: (f32, f32),
    pub velocity: (f32, f32),
}

impl Robot {
    pub fn new(team: Team, position: (f32, f32)) -> Self {
        Self {
            team,
            position,
            velocity: (0f32, 0f32)
        }
    }
    pub fn new_at_origin(team: Team) -> Self {
        Self {
            team,
            position: (0f32, 0f32),
            velocity: (0f32, 0f32)
        }
    }

    pub fn add_velocity(&mut self, velocity: (f32, f32)) {
        self.velocity.0 += velocity.0;
        self.velocity.1 += velocity.1;
    }

    pub fn update_position(&mut self, delta: f32) {
        self.position.0 += self.velocity.0 * delta;
        self.position.1 += self.velocity.1 * delta;
    }
}