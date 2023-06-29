use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Robots {
    pub robots: Vec<Robot>,
}

#[derive(Serialize, Deserialize)]
pub struct Robot {
    pub id: u8,
    pub team: bool,
    pub height: f32,
    pub shape: Shape
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Shape {
    Circle { radius: f32 },
}