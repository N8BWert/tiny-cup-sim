use eframe::{egui::Rect, epaint::Pos2};

use ndarray::{array, Array1};

use crate::parser::{dimensions::Dimensions, robot_init::Shape};

use super::dynamic::Dynamic;

#[derive(Clone, Copy, Debug)]
pub enum Team {
    Red,
    Blue,
}

/// Rotation is in degrees
pub struct Robot {
    robot_id: u8,
    position: Array1<f32>,
    rotation: f32,
    linear_velocity: Array1<f32>,
    angular_velocity: f32,
    shape: Shape,
    team: Team,
}

impl Robot {
    pub fn new(position: Array1<f32>, rotation: f32, shape: Shape, team: Team, robot_id: u8) -> Self {
        Self {
            robot_id,
            position,
            rotation,
            linear_velocity: array![0.0, 0.0],
            angular_velocity: 0f32,
            shape,
            team,
        }
    }

    pub fn robot_state(&self) -> RobotState {
        RobotState {
            robot_id: self.robot_id,
            position: self.position.clone(),
            rotation: self.rotation,
            shape: self.shape,
            team: self.team,
        }
    }
}

impl Dynamic for Robot {
    fn apply_local_linear_velocity(&mut self, linear_velocity: Array1<f32>) {
        self.linear_velocity = linear_velocity;
    }

    fn apply_local_angular_velocity(&mut self, angular_velocity: f32) {
        self.angular_velocity = angular_velocity;
    }

    fn update(&mut self, update_rate: u128) {
        self.position = &self.position + ((update_rate as f32) / 1000.0) * &self.linear_velocity;
        self.rotation = self.rotation + ((update_rate as f32) / 1000.0) * self.angular_velocity;
    }
}

#[derive(Clone, Debug)]
pub struct RobotState {
    pub robot_id: u8,
    pub position: Array1<f32>,
    pub rotation: f32,
    pub shape: Shape,
    pub team: Team,
}

impl RobotState {
    pub fn get_rect(&self, dimensions: &Dimensions, midpoint: &Array1<f32>) -> Rect {
        let point = &self.position * dimensions.ui_dimensions.multiplier;
        let screen_point = point + midpoint;

        let (left_point, right_point) = match self.shape {
            Shape::Circle { radius } => {
                (
                    &screen_point - array![radius, radius] * dimensions.ui_dimensions.multiplier,
                    screen_point + array![radius, radius] * dimensions.ui_dimensions.multiplier,
                )
            }
        };

        let left_point = Pos2::new(
            left_point.get(0).unwrap().to_owned(),
            left_point.get(1).unwrap().to_owned(),
        );

        let right_point = Pos2::new(
            right_point.get(0).unwrap().to_owned(),
            right_point.get(1).unwrap().to_owned(),
        );

        Rect::from_two_pos(left_point, right_point)
    }
}