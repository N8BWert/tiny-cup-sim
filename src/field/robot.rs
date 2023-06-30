use eframe::{egui::Rect, epaint::Pos2};

use ndarray::{array, Array1};

use crate::parser::{dimensions::Dimensions, robot_init::Shape};

pub struct Robot {
    position: Array1<f32>,
    rotation: f32,
    linear_velocity: Array1<f32>,
    angular_velocity: f32,
    shape: Shape,
}

impl Robot {
    pub fn new(position: Array1<f32>, rotation: f32, shape: Shape) -> Self {
        Self {
            position,
            rotation,
            linear_velocity: array![0.0, 0.0],
            angular_velocity: 0f32,
            shape: shape
        }
    }

    pub fn robot_state(&self) -> RobotState {
        RobotState {
            position: self.position.clone(),
            rotation: self.rotation,
            shape: self.shape,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RobotState {
    pub position: Array1<f32>,
    pub rotation: f32,
    pub shape: Shape,
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