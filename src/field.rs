use std::fs;
use std::process::exit;
use toml;

pub mod robot;

use crate::dimensions::Dimensions;
use robot::Robot;

pub struct FieldState {
    pub dimensions: Dimensions,
    pub blue_team_robots: Vec<Robot>,
    pub red_team_robots: Vec<Robot>,
}

impl FieldState {
    pub fn new() -> Self {
        let filename = "dimensions.toml";

        let contents = match fs::read_to_string(filename) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("Could not read file {}", filename);
                exit(1);
            }
        };

        let dimensions: Dimensions = match toml::from_str(contents.as_str()) {
            Ok(dimensions) => dimensions,
            Err(_) => {
                eprintln!("Unable to load data from {}", filename);
                exit(1);
            }
        };

        let filename = "robots.json";

        let contents = match fs::read_to_string(filename) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("Could not read file {}", filename);
                exit(1);
            }
        };

        let robots: crate::Robots = match serde_json::from_str(contents.as_str()) {
            Ok(robots) => robots,
            Err(_) => {
                eprintln!("Unable to load data from {}", filename);
                exit(1);
            }
        };

        let mut blue_team_robots = Vec::with_capacity(3);
        let mut red_team_robots = Vec::with_capacity(3);

        Self {
            dimensions,
            blue_team_robots,
            red_team_robots
        }
    }
}