use serde_derive::{Deserialize};

#[derive(Deserialize, Clone)]
pub struct Dimensions {
    pub field_dimensions: FieldDimensions,
    pub robot_max_dimensions: RobotMaxDimensions,
    pub ball_dimensions: BallDimensions,
    pub goal_dimensions: GoalDimensions,
}

#[derive(Deserialize, Clone)]
pub struct FieldDimensions {
    pub width: f32,
    pub length: f32,
    pub corner_radius: f32,
    pub center_circle_radius: f32,
}

#[derive(Deserialize, Clone)]
pub struct RobotMaxDimensions {
    pub max_length: f32,
    pub max_width: f32,
    pub max_height: f32,
}

#[derive(Deserialize, Clone)]
pub struct BallDimensions {
    pub radius: f32,
}

#[derive(Deserialize, Clone)]
pub struct GoalDimensions {
    pub length: f32,
    pub height: f32,
    pub depth: f32,
}