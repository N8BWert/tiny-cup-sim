use ncomm::publisher_subscriber::{local::{LocalPublisher, LocalSubscriber}, Subscribe, Publish};
use ncomm::node::Node;

#[derive(Clone, Debug)]
pub struct FieldState {
    pub ball_position: [f32; 2],
    pub red_robot_positions: [[f32; 2]; 2],
    pub blue_robot_positions: [[f32; 2]; 2],
}

impl FieldState {
    pub const fn new(
        ball_position: [f32; 2],
        red_robot_positions: [[f32; 2]; 2],
        blue_robot_positions: [[f32; 2]; 2]
    ) -> Self {
        Self {
            ball_position,
            red_robot_positions,
            blue_robot_positions,
        }
    }
}

pub struct FieldStatePublisher {
    field_state: FieldState,
    field_state_publisher: LocalPublisher<FieldState>,
}

impl FieldStatePublisher {
    pub fn new(
        ball_position: [f32; 2],
        red_robot_positions: [[f32; 2]; 2],
        blue_robot_positions: [[f32; 2]; 2],
    ) -> Self {
        Self {
            field_state: FieldState::new(ball_position, red_robot_positions, blue_robot_positions),
            field_state_publisher: LocalPublisher::new(),
        }
    }

    pub fn create_subscriber(&mut self) -> LocalSubscriber<FieldState> {
        self.field_state_publisher.create_subscriber()
    }

    pub fn publish_field_state(&mut self) {
        self.field_state_publisher.send(self.field_state.clone());
    }
}

impl Node for FieldStatePublisher {
    fn name(&self) -> String { "Field State Publisher".into() }

    fn get_update_rate(&self) -> u128 { (1000.0 / 60.0) as u128 }

    fn start(&mut self) {
        self.publish_field_state();
    }

    fn update(&mut self) {
        self.publish_field_state();
    }

    fn shutdown(&mut self) {
        self.publish_field_state();
    }

    fn debug(&self) -> String {
        format!("Field State Publisher\n{:?}", self.field_state)
    }
}