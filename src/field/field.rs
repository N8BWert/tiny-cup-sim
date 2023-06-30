use ndarray::{Array1, Array2, Axis};

use crate::parser::robot_init::Shape;

use super::robot::{Robot, RobotState};
use super::ball::{Ball, BallState};

pub struct Field {
    ball: Ball,
    red_robots: [Robot; 2],
    blue_robots: [Robot; 2],
}

impl Field {
    pub fn new(
        ball_position: Array1<f32>,
        robot_positions: Array2<f32>,
        robot_rotations: Array1<f32>,
        robot_shapes: [Shape; 4],
    ) -> Self {
        Self {
            ball: Ball::new(ball_position),
            red_robots: [
                Robot::new(
                    robot_positions.index_axis(Axis(0), 0).to_owned(),
                    robot_rotations.get(0).unwrap().to_owned(),
                    robot_shapes[0],
                ),
                Robot::new(
                    robot_positions.index_axis(Axis(0), 1).to_owned(),
                    robot_rotations.get(1).unwrap().to_owned(),
                    robot_shapes[1],
                )
            ],
            blue_robots: [
                Robot::new(
                    robot_positions.index_axis(Axis(0), 2).to_owned(),
                    robot_rotations.get(2).unwrap().to_owned(),
                    robot_shapes[2],
                ),
                Robot::new(
                    robot_positions.index_axis(Axis(0), 3).to_owned(),
                    robot_rotations.get(3).unwrap().to_owned(),
                    robot_shapes[3],
                )
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

#[derive(Clone, Debug)]
pub struct FieldState {
    pub ball_state: BallState,
    pub red_robot_states: [RobotState; 2],
    pub blue_robot_states: [RobotState; 2],
}