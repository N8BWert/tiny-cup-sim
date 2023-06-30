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
    pub fn get_rect(&self, dimensions: &Dimensions, midpoint: &Array1<f32>) -> Rect {
        let point = self.position.clone() * dimensions.ui_dimensions.multiplier;
        let screen_point = point + midpoint;

        let left_point = &screen_point - array![dimensions.ball_dimensions.radius, dimensions.ball_dimensions.radius];
        let right_point = screen_point + array![dimensions.ball_dimensions.radius, dimensions.ball_dimensions.radius];

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