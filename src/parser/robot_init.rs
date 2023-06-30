use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct RobotInit {
    pub robots: [Robot; 4],
}

impl RobotInit {
    pub fn get_robot_positions(&self) -> [[f32; 2]; 4] {
        let mut robot_positions = [[0f32; 2]; 4];

        for robot in self.robots {
            robot_positions[robot.id as usize] = robot.start_position;
        }

        robot_positions
    }

    pub fn get_robot_rotations(&self) -> [f32; 4] {
        let mut robot_rotations = [0f32; 4];

        for robot in self.robots {
            robot_rotations[robot.id as usize] = robot.start_rotation;
        }

        robot_rotations
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
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