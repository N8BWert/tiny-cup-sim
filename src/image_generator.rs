use std::{fs, path::Path};

use crate::parser::{robot_init::{RobotInit, Shape}, dimensions::Dimensions};

mod ball;
mod field;
mod goal;
mod robot;

pub fn generate_images(args: Vec<String>, dimensions: &Dimensions, robots: &RobotInit) {
    if let Ok(b) = ensure_file_structure() {
        if b || args.len() >= 2 && args[1] == "build" {
            ball::create_ball_sprite(dimensions.ball_dimensions.radius as u32);
            ball::create_ball_collider(dimensions.ball_dimensions.radius as u32);
            goal::create_goal_sprites(dimensions.goal_dimensions.length as u32, dimensions.goal_dimensions.depth as u32);
            field::create_field_sprite(dimensions.field_dimensions.length as u32, dimensions.field_dimensions.width as u32, dimensions.field_dimensions.corner_radius as u32, dimensions.goal_dimensions.depth as u32);
            field::create_field_collider(dimensions.field_dimensions.length as u32, dimensions.field_dimensions.width as u32, dimensions.field_dimensions.corner_radius as u32, dimensions.goal_dimensions.depth as u32);

            for robot in robots.robots {
                match robot.shape {
                    Shape::Circle{ radius } => {
                        robot::create_circle_robot_sprite(robot.team, (radius * 2.0) as u32, robot.id);
                        robot::create_circle_robot_collider(robot.team, (radius * 2.0) as u32, robot.id)
                    }
                }
            }
        }
    }
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