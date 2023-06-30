use ncomm::{node::Node, publisher_subscriber::{local::{LocalPublisher, LocalSubscriber}, Subscribe, Publish}};

mod robot;

mod ball;

pub mod field;
use field::{Field, FieldState};

pub struct FieldNode {
    field: Field,
    field_state_publisher: LocalPublisher<FieldState>,
}

impl FieldNode {
    pub fn new(
        ball_positions: [f32; 2],
        robot_positions: [[f32; 2]; 4],
        robot_rotations: [f32; 4],
    ) -> Self {
        Self {
            field: Field::new(ball_positions, robot_positions, robot_rotations),
            field_state_publisher: LocalPublisher::new(),
        }
    }

    pub fn create_field_subscriber(&mut self) -> LocalSubscriber<FieldState> {
        self.field_state_publisher.create_subscriber()
    }

    pub fn publish_field_state(&self) {
        self.field_state_publisher.send(self.field.field_state());
    }
}

impl Node for FieldNode {
    fn name(&self) -> String { "Field Node".into() }

    fn get_update_rate(&self) -> u128 { (1000.0 / 60.0) as u128 }

    fn start(&mut self) {
        self.publish_field_state()
    }

    fn update(&mut self) {
        self.publish_field_state()
    }

    fn shutdown(&mut self) {
        self.publish_field_state()
    }

    fn debug(&self) -> String {
        format!("Field Node: {:?}", self.field.field_state())
    }
}