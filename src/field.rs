pub struct Field {
    red_robots: [Robot; 2],
    blue_robots: [Robot; 2],
    ball: Ball,
}

pub struct Robot {
    pub position: [f32; 2],
    pub linear_velocity: [f32; 2],
    pub angular_velocity: f32,
}

pub struct Ball {
    pub position: [f32; 2],
    pub linear_velocity: [f32; 2],
}