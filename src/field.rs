use ncomm::{node::Node, publisher_subscriber::{local::{LocalPublisher, LocalSubscriber}, Subscribe, Publish, Receive}};

use ndarray::{Array1, Array2, array};

mod robot;

mod ball;

pub mod field;
use field::{Field, FieldState};

use crate::ui::ui_node::State;

pub struct FieldNode {
    field: Field,
    field_state_publisher: LocalPublisher<FieldState>,
    state_subscriber: Option<LocalSubscriber<State>>,
}

impl FieldNode {
    pub fn new(
        ball_position: [f32; 2],
        robot_positions: Array2<f32>,
        robot_rotations: Array1<f32>,
    ) -> Self {
        Self {
            field: Field::new(array![ball_position[0], ball_position[1]], robot_positions, robot_rotations),
            field_state_publisher: LocalPublisher::new(),
            state_subscriber: None,
        }
    }

    pub fn create_field_subscriber(&mut self) -> LocalSubscriber<FieldState> {
        self.field_state_publisher.create_subscriber()
    }

    pub fn publish_field_state(&self) {
        self.field_state_publisher.send(self.field.field_state());
    }

    pub fn add_state_subscriber(&mut self, state_subscriber: LocalSubscriber<State>) {
        self.state_subscriber = Some(state_subscriber);
    }
}

impl Node for FieldNode {
    fn name(&self) -> String { "Field Node".into() }

    fn get_update_rate(&self) -> u128 { (1000.0 / 60.0) as u128 }

    fn start(&mut self) {
        self.publish_field_state();

        if let Some(state_subscriber) = self.state_subscriber.as_mut() {
            state_subscriber.update_data();
        }
    }

    fn update(&mut self) {
        self.publish_field_state();

        if let Some(state_subscriber) = self.state_subscriber.as_mut() {
            state_subscriber.update_data();
        }
    }

    fn shutdown(&mut self) {
        self.publish_field_state();

        if let Some(state_subscriber) = self.state_subscriber.as_mut() {
            state_subscriber.update_data();
        }
    }

    fn debug(&self) -> String {
        format!("Field Node: {:?}", self.field.field_state())
    }
}