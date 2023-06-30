use serde_derive::{Serialize, Deserialize};

use ndarray::prelude::*;
use ndarray::{stack, Axis};

#[derive(Serialize, Deserialize, Clone)]
pub struct RobotInit {
    pub robots: [Robot; 4],
}

impl RobotInit {
    pub fn get_robot_positions(&self) -> Array2<f32> {
        stack![
            Axis(0),
            self.robots[0].start_position,
            self.robots[1].start_position,
            self.robots[2].start_position,
            self.robots[3].start_position,
        ]
    }

    pub fn get_robot_rotations(&self) -> Array1<f32> {
        array![
            self.robots[0].start_rotation,
            self.robots[1].start_rotation,
            self.robots[2].start_rotation,
            self.robots[3].start_rotation,
        ]
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Robot {
    pub id: u8,
    pub team: bool,
    pub height: f32,
    pub shape: Shape,
    pub start_position: [f32; 2],
    pub start_rotation: f32,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(tag = "type")]
pub enum Shape {
    Circle { radius: f32 },
}