use std::env;

use eframe::egui;

mod ui_node;
use ui_node::{UINode, UIApp};

mod field;

mod field_state;
use field_state::FieldStatePublisher;

mod parser;
use parser::{get_dimensions, get_robots};

mod image_generator;

fn main() -> Result<(), eframe::Error> {
    let args: Vec<_> = env::args().collect();

    let dimensions = get_dimensions();
    let robots = get_robots();

    image_generator::generate_images(args, &dimensions, &robots);

    let mut field_state_publisher = FieldStatePublisher::new(
        dimensions.ball_dimensions.start_position,
        [[10.0, 10.0], [200.0, 200.0]],
        [[900.0, 900.0], [600.0, 600.0]],
    );

    let ui_node = UINode::new(
        dimensions.field_dimensions.length,
        dimensions.field_dimensions.width,
        [[10.0, 10.0], [200.0, 200.0]],
        [[900.0, 900.0], [600.0, 600.0]],
        field_state_publisher.create_subscriber(),
    );

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(dimensions.field_dimensions.length, dimensions.field_dimensions.width + 50.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Tiny-Cup",
        options,
        Box::new(|_cc| Box::new(
            UIApp::new(
                dimensions,
                50.0,
                ui_node
            )
        )),
    )
}
