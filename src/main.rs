use std::env;

use eframe::egui;

mod ui;
use ui::UIApp;

mod field;
use field::FieldNode;

mod parser;
use parser::{get_dimensions, get_robots};

mod image_generator;

fn main() -> Result<(), eframe::Error> {
    let args: Vec<_> = env::args().collect();

    let dimensions = get_dimensions();
    let robots = get_robots();

    image_generator::generate_images(args, &dimensions, &robots);

    let mut field_node = FieldNode::new(
        dimensions.ball_dimensions.start_position,
        robots.get_robot_positions(),
        robots.get_robot_rotations(),
    );

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(dimensions.field_dimensions.length, dimensions.field_dimensions.width + 50.0)),
        ..Default::default()
    };

    let field_state_subscriber = field_node.create_field_subscriber();
    eframe::run_native(
        "Tiny-Cup",
        options,
        Box::new(|_cc| Box::new(
            UIApp::new(
                dimensions,
                field_state_subscriber,
            )
        )),
    )
}
