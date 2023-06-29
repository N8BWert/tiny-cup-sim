use std::{env, fs, process::exit, path::Path};

mod image_generate;

mod field;
use field::Dimensions;

use robots::{Robots, Shape};

mod robots;

fn main() {
    let args: Vec<_> = env::args().collect();

    let dimensions = get_dimensions();
    let robots = get_robots();

    if let Ok(b) = ensure_file_structure() {
        if b || (args.len() >= 2 && args[1] == "build") {
            image_generate::create_ball_sprite(dimensions.ball_dimensions.radius as u32);
            image_generate::create_ball_collider(dimensions.ball_dimensions.radius as u32);
            image_generate::create_red_goal_sprite(dimensions.goal_dimensions.length as u32, dimensions.goal_dimensions.depth as u32);
            image_generate::create_blue_goal_sprite(dimensions.goal_dimensions.length as u32, dimensions.goal_dimensions.depth as u32);
            image_generate::create_field_sprite(dimensions.field_dimensions.length as u32, dimensions.field_dimensions.width as u32, dimensions.field_dimensions.corner_radius as u32, dimensions.goal_dimensions.depth as u32);
            image_generate::create_field_collider(dimensions.field_dimensions.length as u32, dimensions.field_dimensions.width as u32, dimensions.field_dimensions.corner_radius as u32, dimensions.goal_dimensions.depth as u32);

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
}

fn get_dimensions() -> Dimensions {
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

    return dimensions;
}

fn get_robots() -> Robots {
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

    return robots;
}

fn ensure_file_structure() -> std::io::Result<bool> {
    let mut created_dir = false;

    let image_dir = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/images"));
    if !image_dir.is_dir() {
        fs::create_dir(image_dir)?;
        created_dir = true
    }

    let ball_dir = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/images/ball"));
    if !ball_dir.is_dir() {
        fs::create_dir(ball_dir)?;
        created_dir = true;
    }

    let field_dir = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/images/field"));
    if !field_dir.is_dir() {
        fs::create_dir(field_dir)?;
        created_dir = true;
    }

    let goal_dir = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/images/goal"));
    if !goal_dir.is_dir() {
        fs::create_dir(goal_dir)?;
        created_dir = true;
    }

    let robot_dir = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/images/robot"));
    if !robot_dir.is_dir() {
        fs::create_dir(robot_dir)?;
        created_dir = true;
    }

    let blue_dir = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/images/robot/blue"));
    if !blue_dir.is_dir() {
        fs::create_dir(blue_dir)?;
        created_dir = true;
    }

    let red_dir = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/images/robot/red"));
    if !red_dir.is_dir() {
        fs::create_dir(red_dir)?;
        created_dir = true;
    }

    Ok(created_dir)
}
