use super::robot::{Robot, RobotState};
use super::ball::{Ball, BallState};

pub struct Field {
    ball: Ball,
    red_robots: [Robot; 2],
    blue_robots: [Robot; 2],
}

impl Field {
    pub const fn new(
        ball_position: [f32; 2],
        robot_positions: [[f32; 2]; 4],
        robot_rotations: [f32; 4],
    ) -> Self {
        Self {
            ball: Ball::new(ball_position),
            red_robots: [
                Robot::new(robot_positions[0], robot_rotations[0]),
                Robot::new(robot_positions[1], robot_rotations[1]),
            ],
            blue_robots: [
                Robot::new(robot_positions[2], robot_rotations[2]),
                Robot::new(robot_positions[3], robot_rotations[3]),
            ]
        }
    }

    pub fn field_state(&self) -> FieldState {
        FieldState {
            ball_state: self.ball.ball_state(),
            red_robot_states: [self.red_robots[0].robot_state(), self.red_robots[1].robot_state()],
            blue_robot_states: [self.blue_robots[0].robot_state(), self.blue_robots[1].robot_state()],
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct FieldState {
    pub ball_state: BallState,
    pub red_robot_states: [RobotState; 2],
    pub blue_robot_states: [RobotState; 2],
}