use std::env;

use eframe::egui;

mod ui;
use ncomm::node::Node;
use ui::{UIApp, ui_node::UINode};

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
        robots.get_robot_shapes(),
    );

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(dimensions.ui_dimensions.length, dimensions.ui_dimensions.height)),
        ..Default::default()
    };

    let mut ui_node = UINode::new(field_node.create_field_subscriber());

    field_node.add_state_subscriber(ui_node.create_state_subscriber());

    field_node.start();
    field_node.update();

    eframe::run_native(
        "Tiny-Cup",
        options,
        Box::new(|_cc| Box::new(
            UIApp::new(
                dimensions,
                ui_node,
            )
        )),
    )
}
