use eframe::{egui::Rect, epaint::Pos2};

use ndarray::{array, Array1};

use crate::parser::dimensions::Dimensions;

pub struct Ball {
    position: Array1<f32>,
    linear_velocity: Array1<f32>,
}

impl Ball {
    pub fn new(position: Array1<f32>) -> Self {
        Self {
            position,
            linear_velocity: array![0.0, 0.0],
        }
    }

    pub fn ball_state(&self) -> BallState {
        BallState {
            position: self.position.clone()
        }
    }
}

#[derive(Clone, Debug)]
pub struct BallState {
    pub position: Array1<f32>,
}

impl BallState {
    pub fn get_rect(&self, dimensions: &Dimensions) -> Rect {


        let left_point = Pos2::new(
            (self.position[0] - dimensions.ball_dimensions.radius) * dimensions.ui_dimensions.multiplier,
            0.0,
        );

        let right_point = Pos2::new(
            100.0,
            100.0,
        );

        Rect::from_two_pos(left_point, right_point)
    }
}