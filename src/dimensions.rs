use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Dimensions {
    field_dimensions: FieldDimensions,
    robot_max_dimensions: RobotMaxDimensions,
    ball_dimensions: BallDimensions,
    goal_dimensions: GoalDimensions,
    goal_zone_dimensions: GoalZoneDimensions,
}

#[derive(Deserialize)]
pub struct FieldDimensions {
    width: f32,
    length: f32,
    field_bevel_radius: f32,
    center_circle_radius: f32,
    behind_goal_width: f32,
}

#[derive(Deserialize)]
pub struct RobotMaxDimensions {
    max_length: f32,
    max_width: f32,
    max_height: f32,
}

#[derive(Deserialize)]
pub struct BallDimensions {
    radius: f32,
}

#[derive(Deserialize)]
pub struct GoalDimensions {
    length: f32,
    height: f32,
    depth: f32,
}

#[derive(Deserialize)]
pub struct GoalZoneDimensions {
    radius: f32,
}
