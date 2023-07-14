use ncomm::node::Node;
use ncomm::publisher_subscriber::Receive;
use ncomm::publisher_subscriber::{Subscribe, Publish, local::{LocalPublisher, LocalSubscriber}, udp::UdpSubscriber};

use ndarray::{Array1, Array2, array};

pub mod robot;
use robot::Team;

pub mod ball;

pub mod field;
use field::{Field, FieldState};

mod dynamic;
use dynamic::Dynamic;

use crate::ui::ui_node::State;
use crate::parser::{robot_init::Shape, configuration::TeamConfiguration};

#[repr(C)]
#[derive(Clone)]
/// mem::size_of::<MotionCommand>() = 16
/// 
/// TODO: In the future the size should be determined by mem::size_of::<MotionCommand>() in 
/// the motion command subscribers
pub struct RobotMotionCommand {
    robot_id: u8,
    linear_velocity: [f32; 2],
    angular_velocity: f32,
}

#[repr(C)]
#[derive(Clone)]
pub struct MotionCommand {
    motion_commands: [RobotMotionCommand; 3],
}

pub struct FieldNode {
    field: Field,
    field_state_publisher: LocalPublisher<FieldState>,
    state_subscriber: Option<LocalSubscriber<State>>,
    red_motion_command_subscriber: UdpSubscriber<MotionCommand, 48>,
    blue_motion_command_subscriber: UdpSubscriber<MotionCommand, 48>,
}

impl FieldNode {
    pub fn new(
        ball_position: [f32; 2],
        robot_positions: Array2<f32>,
        robot_rotations: Array1<f32>,
        robot_shapes: [Shape; 4],
        blue_team_configuration: &TeamConfiguration,
        red_team_configuration: &TeamConfiguration,
    ) -> Self {
        Self {
            field: Field::new(array![ball_position[0], ball_position[1]], robot_positions, robot_rotations, robot_shapes),
            field_state_publisher: LocalPublisher::new(),
            state_subscriber: None,
            blue_motion_command_subscriber: UdpSubscriber::new(
                blue_team_configuration.network_configuration.bind_address.as_str(),
                blue_team_configuration.network_configuration.listen_address.as_str()
            ),
            red_motion_command_subscriber: UdpSubscriber::new(
                red_team_configuration.network_configuration.bind_address.as_str(),
                red_team_configuration.network_configuration.listen_address.as_str(),
            ),
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
        // Check the state
        if let Some(state_subscriber) = self.state_subscriber.as_mut() {
            state_subscriber.update_data();
        }

        // Only Update the field on Play
        let state = match self.state_subscriber.as_ref() {
            Some(state_subscriber) => state_subscriber.data,
            _ => Some(State::Stop),
        };

        // Update blue team motion commands
        self.blue_motion_command_subscriber.update_data();
        if let Some(motion_command) = self.blue_motion_command_subscriber.data.as_ref() {
            for robot_motion_command in motion_command.motion_commands.iter() {
                self.field.apply_local_robot_velocity(
                    robot_motion_command.robot_id,
                    Team::Blue,
                    array![
                        robot_motion_command.linear_velocity[0],
                        robot_motion_command.linear_velocity[1]
                    ],
                    robot_motion_command.angular_velocity,
                );
            }
        }

        // Update red team motion commands
        self.red_motion_command_subscriber.update_data();
        if let Some(motion_command) = self.red_motion_command_subscriber.data.as_ref() {
            for robot_motion_command in motion_command.motion_commands.iter() {
                self.field.apply_local_robot_velocity(
                    robot_motion_command.robot_id, 
                    Team::Red, 
                    array![
                        robot_motion_command.linear_velocity[0],
                        robot_motion_command.linear_velocity[1],
                    ],
                    robot_motion_command.angular_velocity,
                );
            }
        }

        // Update the state of the field
        if let Some(state) = state {
            match state {
                State::Restart => self.field.reset_field(),
                _ => self.field.update(self.get_update_rate()),
            }
        }

        // Publish the updated field state
        self.publish_field_state();
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