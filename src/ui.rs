use std::path::Path;

use ncomm::{node::Node, publisher_subscriber::{Publish, local::LocalSubscriber}};

use eframe::{egui::{self, Rect, Button, Image}, epaint::{Pos2}};

use egui_extras::RetainedImage;

use crate::parser::dimensions::Dimensions;
use crate::field::field::FieldState;

mod ui_node;
use ui_node::{State, UINode};

pub struct UIApp {
    dimensions: Dimensions,
    background: RetainedImage,
    node: UINode,
}

impl UIApp {
    pub fn new(dimensions: Dimensions, field_state_subscriber: LocalSubscriber<FieldState>) -> Self {
        Self {
            dimensions: dimensions.clone(),
            background: RetainedImage::from_color_image(
                "background",
                load_image_from_path(Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/images/field/field_sprite.png"))).unwrap()
            ),
            node: UINode::new(field_state_subscriber),
        }
    }
}

impl eframe::App for UIApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.node.update();

        egui::CentralPanel::default().show(ctx, |ui| {
            let button_width = self.dimensions.ui_dimensions.length / 8.0;
            let button_height = self.dimensions.ui_dimensions.height - 50.0;

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
            ui.put(Rect::from_two_pos(Pos2::new(0.0, button_height), Pos2::new(self.dimensions.field_dimensions.width, self.dimensions.ui_dimensions.height)),
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