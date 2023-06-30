use eframe::{egui::Rect, epaint::Pos2};

use ndarray::{array, Array1};

use crate::parser::dimensions::Dimensions;

pub struct Robot {
    position: Array1<f32>,
    rotation: f32,
    linear_velocity: Array1<f32>,
    angular_velocity: f32,
}

impl Robot {
    pub fn new(position: Array1<f32>, rotation: f32) -> Self {
        Self {
            position,
            rotation,
            linear_velocity: array![0.0, 0.0],
            angular_velocity: 0f32,
        }
    }

    pub fn robot_state(&self) -> RobotState {
        RobotState {
            position: self.position.clone(),
            rotation: self.rotation,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RobotState {
    pub position: Array1<f32>,
    pub rotation: f32,
}

impl RobotState {
    pub fn get_rect(&self, dimensions: &Dimensions) -> Rect {
        let left_point = Pos2::new(
            0.0,
            10.0
        );

        let right_point = Pos2::new(
            0.0,
            10.0,
        );

        Rect::from_two_pos(left_point, right_point)
    }
}