use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct RobotInit {
    pub robot_states: [Robot; 4],
}

impl RobotInit {
    pub fn get_robot_positions(&self) -> [[f32; 2]; 4] {
        let mut robot_positions = [[0f32; 2]; 4];

        for robot in self.robot_states {
            robot_positions[robot.id as usize] = robot.start_position.clone();
        }

        robot_positions
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Robot {
    pub id: u8,
    pub team: bool,
    pub height: f32,
    pub shape: Shape,
    pub start_position: [f32; 2],
    pub start_rotation: [f32; 2],
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(tag = "type")]
pub enum Shape {
    Circle { radius: f32 },
}