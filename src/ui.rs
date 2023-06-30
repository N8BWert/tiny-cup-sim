use std::{path::Path, process::exit};

use ncomm::{node::Node, publisher_subscriber::Publish};

use eframe::{egui::{self, Rect, Button, Image}, epaint::{Pos2}};

use egui_extras::RetainedImage;

use ndarray::{array, Array1};

use crate::parser::dimensions::Dimensions;

pub mod ui_node;
use ui_node::{State, UINode};

const IMAGE_SPRITE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/images/field/field_sprite.png");
const BALL_SPRITE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/images/ball/ball_sprite.png");
const RED_ROBOT_SPRITE_PATHS: [&str; 2] = [
    concat!(env!("CARGO_MANIFEST_DIR"), "/images/robot/red/robot_zero_sprite.png"),
    concat!(env!("CARGO_MANIFEST_DIR"), "/images/robot/red/robot_one_sprite.png"),
];
const BLUE_ROBOT_SPRITE_PATHS: [&str; 2] = [
    concat!(env!("CARGO_MANIFEST_DIR"), "/images/robot/blue/robot_zero_sprite.png"),
    concat!(env!("CARGO_MANIFEST_DIR"), "/images/robot/blue/robot_one_sprite.png"),
];
const RED_GOAL_SPRITE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/images/goal/red_goal_sprite.png");
const BLUE_GOAL_SPRITE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/images/goal/blue_goal_sprite.png");

pub struct UIApp {
    dimensions: Dimensions,
    background: RetainedImage,
    red_goal_sprite: RetainedImage,
    blue_goal_sprite: RetainedImage,
    ball_sprite: RetainedImage,
    red_robot_sprites: [RetainedImage; 2],
    blue_robot_sprites: [RetainedImage; 2],
    node: UINode,
}

impl UIApp {
    pub fn new(dimensions: Dimensions, ui_node: UINode) -> Self {
        Self {
            dimensions: dimensions.clone(),
            background: RetainedImage::from_color_image(
                "background",
                match load_image_from_path(Path::new(IMAGE_SPRITE_PATH)) {
                    Ok(img) => img,
                    Err(_) => {
                        eprintln!("could not load field sprite");
                        exit(1);
                    }
                }
            ),
            red_goal_sprite: RetainedImage::from_color_image(
                "red goal sprite",
                match load_image_from_path(Path::new(RED_GOAL_SPRITE_PATH)) {
                    Ok(img) => img,
                    Err(_) => {
                        eprintln!("could not load red goal image");
                        exit(1);
                    }
                }
            ),
            blue_goal_sprite: RetainedImage::from_color_image(
                "blue goal sprite",
                match load_image_from_path(Path::new(BLUE_GOAL_SPRITE_PATH)) {
                    Ok(img) => img,
                    Err(_) => {
                        eprintln!("could not load blue goal image");
                        exit(1);
                    }
                }
            ),
            ball_sprite: RetainedImage::from_color_image(
                "ball_sprite",
                match load_image_from_path(Path::new(BALL_SPRITE_PATH)) {
                    Ok(img) => img,
                    Err(_) => {
                        eprintln!("could not load ball sprite");
                        exit(1);
                    }
                }
            ),
            red_robot_sprites: [
                RetainedImage::from_color_image(
                    "red_robot_one_sprite",
                    match load_image_from_path(Path::new(RED_ROBOT_SPRITE_PATHS[0])) {
                        Ok(img) => img,
                        Err(_) => {
                            eprintln!("could not load red robot 0 sprite");
                            exit(1);
                        }
                    }
                ),
                RetainedImage::from_color_image(
                    "red_robot_one_sprite",
                    match load_image_from_path(Path::new(RED_ROBOT_SPRITE_PATHS[1])) {
                        Ok(img) => img,
                        Err(_) => {
                            eprintln!("could not load red robot 1 sprite");
                            exit(1);
                        }
                    }
                ),
            ],
            blue_robot_sprites: [
                RetainedImage::from_color_image(
                    "Blue Robot 1 Sprite",
                    match load_image_from_path(Path::new(BLUE_ROBOT_SPRITE_PATHS[0])) {
                        Ok(img) => img,
                        Err(_) => {
                            eprintln!("could not load blue robot 0 sprite");
                            exit(1);
                        }
                    }
                ),
                RetainedImage::from_color_image(
                    "Blue Robot 2 Sprite",
                    match load_image_from_path(Path::new(BLUE_ROBOT_SPRITE_PATHS[1])) {
                        Ok(img) => img,
                        Err(_) => {
                            eprintln!("could not load blue robot 1 sprite");
                            exit(1);
                        }
                    }
                ),
            ],
            node: ui_node,
        }
    }
}

impl eframe::App for UIApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.node.update();

        egui::CentralPanel::default().show(ctx, |ui| {
            let button_width = self.dimensions.ui_dimensions.length / 7.0;
            let button_height = self.dimensions.ui_dimensions.button_height;

            if ui.put(Rect::from_two_pos(Pos2::new(0.0, 0.0), Pos2::new(button_width, button_height)), Button::new("▶️")).clicked() {
                self.node.state_publisher.send(State::Play);
            }

            if ui.put(Rect::from_two_pos(Pos2::new(2.0 * button_width, 0.0), Pos2::new(3.0 * button_width, button_height)), Button::new("⏸️")).clicked() {
                self.node.state_publisher.send(State::Pause);
            }

            if ui.put(Rect::from_two_pos(Pos2::new(4.0 * button_width, 0.0), Pos2::new(5.0 * button_width, button_height)), Button::new("⏹️")).clicked() {
                self.node.state_publisher.send(State::Stop);
            }

            if ui.put(Rect::from_two_pos(Pos2::new(6.0 * button_width, 0.0), Pos2::new(7.0 * button_width, button_height)), Button::new("🔄")).clicked() {
                self.node.state_publisher.send(State::Restart);
            }

            // Draw the background
            ui.put(
                Rect::from_two_pos(Pos2::new(0.0, button_height), Pos2::new(self.dimensions.ui_dimensions.length, self.dimensions.ui_dimensions.height)),
                Image::new(self.background.texture_id(ctx), self.background.size_vec2())
            );

            let goal_start_y = (self.dimensions.field_dimensions.width - self.dimensions.goal_dimensions.length) / 2.0 * self.dimensions.ui_dimensions.multiplier + button_height;

            // Draw the red goal (left)
            ui.put(
                Rect::from_two_pos(Pos2::new(0.0, goal_start_y), Pos2::new(self.dimensions.goal_dimensions.depth * self.dimensions.ui_dimensions.multiplier, goal_start_y + self.dimensions.goal_dimensions.length * self.dimensions.ui_dimensions.multiplier)),
                Image::new(self.red_goal_sprite.texture_id(ctx), self.red_goal_sprite.size_vec2())
            );

            // Draw the blue goal (right)
            ui.put(
                Rect::from_two_pos(Pos2::new(self.dimensions.ui_dimensions.length - self.dimensions.goal_dimensions.depth * self.dimensions.ui_dimensions.multiplier, goal_start_y), Pos2::new(self.dimensions.ui_dimensions.length, goal_start_y + self.dimensions.goal_dimensions.length * self.dimensions.ui_dimensions.multiplier)),
                Image::new(self.blue_goal_sprite.texture_id(ctx), self.blue_goal_sprite.size_vec2())
            );

            if let Some(field_state) = self.node.field_state_subscriber.data.as_ref() {
                let midpoint: Array1<f32> = array![
                    self.dimensions.field_dimensions.length * self.dimensions.ui_dimensions.multiplier / 2.0,
                    self.dimensions.ui_dimensions.button_height + self.dimensions.field_dimensions.width * self.dimensions.ui_dimensions.multiplier / 2.0,
                ];

                // Draw the red robots
                ui.put(field_state.red_robot_states[0].get_rect(&self.dimensions, &midpoint),
                    Image::new(self.red_robot_sprites[0].texture_id(ctx), self.red_robot_sprites[0].size_vec2()));

                ui.put(field_state.red_robot_states[1].get_rect(&self.dimensions, &midpoint),
                    Image::new(self.red_robot_sprites[1].texture_id(ctx), self.red_robot_sprites[1].size_vec2()));

                // Draw the blue robots
                ui.put(field_state.blue_robot_states[0].get_rect(&self.dimensions, &midpoint),
                    Image::new(self.blue_robot_sprites[0].texture_id(ctx), self.blue_robot_sprites[0].size_vec2()));

                ui.put(field_state.blue_robot_states[1].get_rect(&self.dimensions, &midpoint),
                    Image::new(self.blue_robot_sprites[1].texture_id(ctx), self.blue_robot_sprites[1].size_vec2()));

                // Draw the ball
                ui.put(
                    field_state.ball_state.get_rect(&self.dimensions, &midpoint),
                    Image::new(self.ball_sprite.texture_id(ctx), self.ball_sprite.size_vec2())
                );
            }
        });
    }
}

pub fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()))
}