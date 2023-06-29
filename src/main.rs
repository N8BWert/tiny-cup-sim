use std::{env, fs, process::exit};

mod image_generate;

mod field;
use field::Dimensions;

use robots::{Robots, Shape};

mod robots;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() >= 2 && args[1] == "build" {
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

        image_generate::create_ball_sprite(dimensions.ball_dimensions.radius as u32);
        image_generate::create_ball_collider(dimensions.ball_dimensions.radius as u32);
        image_generate::create_red_goal_sprite(dimensions.goal_dimensions.length as u32, dimensions.goal_dimensions.depth as u32);
        image_generate::create_blue_goal_sprite(dimensions.goal_dimensions.length as u32, dimensions.goal_dimensions.depth as u32);
        image_generate::create_field_sprite(dimensions.field_dimensions.length as u32, dimensions.field_dimensions.width as u32, dimensions.field_dimensions.corner_radius as u32, dimensions.goal_dimensions.depth as u32);
        image_generate::create_field_collider(dimensions.field_dimensions.length as u32, dimensions.field_dimensions.width as u32, dimensions.field_dimensions.corner_radius as u32, dimensions.goal_dimensions.depth as u32);

        let filename = "robots.json";
        
        let contents = match fs::read_to_string(filename) {
            Ok(file) => file,
            Err(_) => {
                eprint!("Could not read file {}", filename);
                exit(1);
            }
        };

        let robots: Robots = match serde_json::from_str(contents.as_str()) {
            Ok(robots) => robots,
            Err(_) => {
                eprint!("Unable to load data from {}", filename);
                exit(1);
            }
        };

        for robot in robots.robots {
            match robot.shape {
                Shape::Circle{ radius } => {
                    image_generate::create_circle_robot_sprite(robot.team, (radius * 2.0) as u32, robot.id);
                    image_generate::create_circle_robot_collider(robot.team, (radius * 2.0) as u32, robot.id)
                }
            }
        }
    }
}
