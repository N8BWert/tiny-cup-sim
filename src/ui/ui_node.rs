use ncomm::{node::Node, publisher_subscriber::{local::{LocalSubscriber, LocalPublisher}, Receive, Subscribe, Publish}};

use crate::field::{field::FieldState};

#[derive(Copy, Clone, Debug)]
pub enum State {
    Play,
    Pause,
    Stop,
    Restart,
}

pub struct UINode {
    pub state_publisher: LocalPublisher<State>,
    pub field_state_subscriber: LocalSubscriber<FieldState>,
}

impl UINode {
    pub fn new(field_state_subscriber: LocalSubscriber<FieldState>) -> Self {
        Self {
            state_publisher: LocalPublisher::new(),
            field_state_subscriber,
        }
    }

    pub fn create_state_subscriber(&mut self) -> LocalSubscriber<State> {
        self.state_publisher.create_subscriber()
    }

    pub fn publish_state(&mut self, state: State) {
        self.state_publisher.send(state);
    }
}

impl Node for UINode {
    fn name(&self) -> String { "UI Node".into() }

    // Doesn't matter we're going to use the eframe update function
    fn get_update_rate(&self) -> u128 { 16 }

    fn start(&mut self) {
        self.state_publisher.send(State::Pause);
        self.field_state_subscriber.update_data();
    }

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