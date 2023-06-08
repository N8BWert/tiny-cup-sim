use crate::sim_field::field_representations::team::Team;

pub struct Robot {
    pub team: Team,
    pub position: (f64, f64),
    pub velocity: (f64, f64),
}

impl Robot {
    pub fn new(team: Team, position: (f64, f64)) -> Self {
        Self {
            team,
            position,
            velocity: (0f64, 0f64)
        }
    }
    pub fn new_at_origin(team: Team) -> Self {
        Self {
            team,
            position: (0f64, 0f64),
            velocity: (0f64, 0f64)
        }
    }

    pub fn add_velocity(&mut self, velocity: (f64, f64)) {
        self.velocity.0 += velocity.0;
        self.velocity.1 += velocity.1;
    }

    /// Update the position of this robot by a delta (ms)
    pub fn update_position(&mut self, delta: u128) {
        self.position.0 += self.velocity.0 * (delta as f64) * 0.001;
        self.position.1 += self.velocity.1 * (delta as f64) * 0.001;
    }
}