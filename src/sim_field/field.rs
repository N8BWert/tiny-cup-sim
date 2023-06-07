use crate::sim_field::{ball::Ball, robot::Robot, team::Team};

pub struct Field {
    ball: Ball,
    blue_team_robots: Vec<Robot>,
    red_team_robots: Vec<Robot>
}

impl Field {
    pub fn new(ball_radius: u8) -> Self {
        let mut blue_team_robots = Vec::with_capacity(3);
        for _ in 0..3 {
            blue_team_robots.push(Robot::new_at_origin(Team::Blue));
        }

        let mut red_team_robots = Vec::with_capacity(3);
        for _ in 0..3 {
            red_team_robots.push(Robot::new_at_origin(Team::Red));
        }

        Self {
            ball: Ball::new_at_origin(ball_radius),
            blue_team_robots,
            red_team_robots
        }
    }

    pub fn new_with_positions(ball_radius: u8, ball_location: (f32, f32), blue_team_positions: Vec<(f32, f32)>, red_team_positions: Vec<(f32, f32)>) -> Self {
        let mut blue_team_robots = Vec::with_capacity(3);
        for position in blue_team_positions {
            blue_team_robots.push(Robot::new(Team::Blue, position));
        }

        let mut red_team_robots = Vec::with_capacity(3);
        for position in red_team_positions {
            red_team_robots.push(Robot::new(Team::Red, position));
        }

        Self {
            ball: Ball::new(ball_radius, ball_location),
            blue_team_robots,
            red_team_robots
        }
    }
}