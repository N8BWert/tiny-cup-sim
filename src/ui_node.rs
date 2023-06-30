use std::path::Path;

use ncomm::{node::Node, publisher_subscriber::{local::{LocalSubscriber, LocalPublisher}, Receive, Subscribe, Publish}};

use eframe::{egui::{self, Rect, Button, Image}, epaint::{Pos2}};

use egui_extras::RetainedImage;

use crate::{field_state::FieldState, dimensions::Dimensions};

#[derive(Debug, Clone)]
pub enum State {
    Play,
    Pause,
    Stop,
    Restart,
}

pub struct UIApp {
    _dimensions: Dimensions,
    ui_height: f32,
    background: RetainedImage,
    node: UINode,
}

impl UIApp {
    pub fn new(dimensions: Dimensions, button_height: f32, ui_node: UINode) -> Self {
        Self {
            _dimensions: dimensions.clone(),
            ui_height: dimensions.field_dimensions.width + button_height,
            background: RetainedImage::from_color_image(
                "background",
                load_image_from_path(Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/images/field/field_sprite.png"))).unwrap()
            ),
            node: ui_node,
        }
    }
}

impl eframe::App for UIApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.node.update();

        egui::CentralPanel::default().show(ctx, |ui| {
            let button_width = self.node.field_length / 8.0;
            let button_height = self.node.field_width - self.ui_height;

            if ui.put(Rect::from_two_pos(Pos2::new(0.0, 0.0), Pos2::new(button_width, button_height)), Button::new("")).clicked() {
                self.node.state_publisher.send(State::Play);
            }

            if ui.put(Rect::from_two_pos(Pos2::new(2.0 * button_width, 0.0), Pos2::new(3.0 * button_width, button_height)), Button::new("")).clicked() {
                self.node.state_publisher.send(State::Pause);
            }

            if ui.put(Rect::from_two_pos(Pos2::new(4.0 * button_width, 0.0), Pos2::new(5.0 * button_width, button_height)), Button::new("")).clicked() {
                self.node.state_publisher.send(State::Stop);
            }

            if ui.put(Rect::from_two_pos(Pos2::new(7.0 * button_width, 0.0), Pos2::new(8.0 * button_width, button_height)), Button::new("")).clicked() {
                self.node.state_publisher.send(State::Restart);
            }

            // TODO: add image
            ui.put(Rect::from_two_pos(Pos2::new(0.0, button_height), Pos2::new(self.node.field_length, self.ui_height)),
                Image::new(self.background.texture_id(ctx), self.background.size_vec2())
            );
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

pub struct UINode {
    pub field_length: f32,
    pub field_width: f32,
    pub state_publisher: LocalPublisher<State>,
    pub field_state_subscriber: LocalSubscriber<FieldState>,
}

impl UINode {
    pub const fn new(
        field_length: f32,
        field_width: f32,
        red_robot_start_positions: [[f32; 2]; 2],
        blue_robot_start_positions: [[f32; 2]; 2],
        mut field_state_subscriber: LocalSubscriber<FieldState>,
    ) -> Self {
        field_state_subscriber.data = Some(FieldState{
            ball_position: [0.0, 0.0],
            red_robot_positions: red_robot_start_positions,
            blue_robot_positions: blue_robot_start_positions,
        });
        Self {
            field_length,
            field_width,
            state_publisher: LocalPublisher::new(),
            field_state_subscriber
        }
    }

    pub fn create_state_subscriber(&mut self) -> LocalSubscriber<State> {
        self.state_publisher.create_subscriber()
    }
}

impl Node for UINode {
    fn name(&self) -> String { "UI Node".into() }

    // Doesn't matter we're going to use the eframe update function
    fn get_update_rate(&self) -> u128 { 16 }

    fn start(&mut self) {}

    fn update(&mut self) {
        self.field_state_subscriber.update_data();
    }

    fn shutdown(&mut self) {
        self.field_state_subscriber.update_data();
    }

    fn debug(&self) -> String {
        format!("UINode:\n{:?}", self.field_state_subscriber.data)
    }
}