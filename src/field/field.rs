use ndarray::{Array1, Array2, Axis};

use crate::parser::robot_init::Shape;

use super::dynamic::Dynamic;
use super::robot::{Robot, RobotState, Team};
use super::ball::{Ball, BallState};

pub struct Field {
    ball: Ball,
    red_robots: [Robot; 2],
    blue_robots: [Robot; 2],
    red_team_score: u8,
    blue_team_score: u8,
    ball_start_position: Array1<f32>,
    robot_start_positions: Array2<f32>,
    robot_start_rotations: Array1<f32>,
    robot_start_shapes: [Shape; 4],
}

impl Field {
    pub fn new(
        ball_position: Array1<f32>,
        robot_positions: Array2<f32>,
        robot_rotations: Array1<f32>,
        robot_shapes: [Shape; 4],
    ) -> Self {
        Self {
            ball: Ball::new(ball_position.clone()),
            red_robots: [
                Robot::new(
                    robot_positions.index_axis(Axis(0), 0).to_owned(),
                    robot_rotations.get(0).unwrap().to_owned(),
                    robot_shapes[0],
                    Team::Red,
                    0u8,
                ),
                Robot::new(
                    robot_positions.index_axis(Axis(0), 1).to_owned(),
                    robot_rotations.get(1).unwrap().to_owned(),
                    robot_shapes[1],
                    Team::Red,
                    1u8,
                )
            ],
            blue_robots: [
                Robot::new(
                    robot_positions.index_axis(Axis(0), 2).to_owned(),
                    robot_rotations.get(2).unwrap().to_owned(),
                    robot_shapes[2],
                    Team::Blue,
                    0u8,
                ),
                Robot::new(
                    robot_positions.index_axis(Axis(0), 3).to_owned(),
                    robot_rotations.get(3).unwrap().to_owned(),
                    robot_shapes[3],
                    Team::Blue,
                    1u8,
                )
            ],
            red_team_score: 0u8,
            blue_team_score: 0u8,
            ball_start_position: ball_position,
            robot_start_positions: robot_positions,
            robot_start_rotations: robot_rotations,
            robot_start_shapes: robot_shapes,
        }
    }

    pub fn field_state(&self) -> FieldState {
        FieldState {
            ball_state: self.ball.ball_state(),
            red_robot_states: [self.red_robots[0].robot_state(), self.red_robots[1].robot_state()],
            blue_robot_states: [self.blue_robots[0].robot_state(), self.blue_robots[1].robot_state()],
        }
    }

    pub fn reset_field(&mut self) {
        self.ball = Ball::new(self.ball_start_position.clone());
        self.red_robots = [
            Robot::new(
                self.robot_start_positions.index_axis(Axis(0), 0).to_owned(),
                self.robot_start_rotations.get(0).unwrap().to_owned(),
                self.robot_start_shapes[0],
                Team::Red,
                0u8,
            ),
            Robot::new(
                self.robot_start_positions.index_axis(Axis(0), 1).to_owned(),
                self.robot_start_rotations.get(1).unwrap().to_owned(),
                self.robot_start_shapes[1],
                Team::Red,
                1u8,
            )
        ];
        self.blue_robots = [
            Robot::new(
                self.robot_start_positions.index_axis(Axis(0), 2).to_owned(),
                self.robot_start_rotations.get(2).unwrap().to_owned(),
                self.robot_start_shapes[2],
                Team::Blue,
                0u8,
            ),
            Robot::new(
                self.robot_start_positions.index_axis(Axis(0), 3).to_owned(),
                self.robot_start_rotations.get(3).unwrap().to_owned(),
            self.robot_start_shapes[3],
                Team::Blue,
                1u8,
            )
        ];
        self.red_team_score = 0u8;
        self.blue_team_score = 0u8;
    }

    pub fn apply_local_robot_velocity(&mut self, robot_id: u8, team: Team, linear_velocity: Array1<f32>, angular_velocity: f32) {
        match team {
            Team::Red => self.red_robots[robot_id as usize].apply_local_velocity(linear_velocity, angular_velocity),
            Team::Blue => self.blue_robots[robot_id as usize].apply_local_velocity(linear_velocity, angular_velocity),
        };
    }
}

impl Dynamic for Field {
    fn update(&mut self, update_rate: u128) {
        for red_robot in self.red_robots.iter_mut() {
            red_robot.update(update_rate);
        }

        for blue_robot in self.blue_robots.iter_mut() {
            blue_robot.update(update_rate);
        }

        self.ball.update(update_rate);
    }
}

#[derive(Clone, Debug)]
pub struct FieldState {
    pub ball_state: BallState,
    pub red_robot_states: [RobotState; 2],
    pub blue_robot_states: [RobotState; 2],
}