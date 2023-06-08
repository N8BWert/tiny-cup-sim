use ncomm::node::Node;
use ncomm::publisher_subscriber::Receive;
use ncomm::publisher_subscriber::udp::UdpSubscriber;

use crate::sim_field::field_representations::field::Field;

#[repr(C)]
#[derive(Clone, Copy)]
struct RobotInstruction {
    pub id: u8,
    pub x: f64,
    pub y: f64
}

pub struct FieldNode<'a> {
    field: Field,
    blue_team_subscribers: Vec<UdpSubscriber<RobotInstruction, 17>>,
    red_team_subscribers: Vec<UdpSubscriber<RobotInstruction, 17>>,
    name: &'a str,
    update_rate: u128,
}

impl<'a> FieldNode<'a> {
    pub fn new(name: &'a str, update_rate: u128, ball_radius: u8,
                blue_binds: Vec<&'a str>, blue_recvs: Vec<&'a str>,
                red_binds: Vec<&'a str>, red_recvs: Vec<&'a str>) -> Self {
        let mut blue_team_subscribers = Vec::with_capacity(3);
        for (bind, recv) in blue_binds.iter().zip(blue_recvs) {
            blue_team_subscribers.push(UdpSubscriber::new(bind, recv));
        }

        let mut red_team_subscribers = Vec::with_capacity(3);
        for (bind, recv) in red_binds.iter().zip(red_recvs) {
            red_team_subscribers.push(UdpSubscriber::new(bind, recv));
        }
        
        Self {
            field: Field::new(ball_radius),
            blue_team_subscribers,
            red_team_subscribers,
            name,
            update_rate
        }
    }

    pub fn new_with_positions(name: &'a str, update_rate: u128,
                              ball_radius: u8, ball_location: (f64, f64),
                              blue_team_positions: Vec<(f64, f64)>,
                              red_team_positions: Vec<(f64, f64)>,
                              blue_binds: Vec<&'a str>, blue_recvs: Vec<&'a str>,
                              red_binds: Vec<&'a str>, red_recvs: Vec<&'a str>) -> Self {

        let mut blue_team_subscribers = Vec::with_capacity(3);
        for (bind, recv) in blue_binds.iter().zip(blue_recvs) {
            blue_team_subscribers.push(UdpSubscriber::new(bind, recv));
        }

        let mut red_team_subscribers = Vec::with_capacity(3);
        for (bind, recv) in red_binds.iter().zip(red_recvs) {
            red_team_subscribers.push(UdpSubscriber::new(bind, recv));
        }

        Self {
            field: Field::new_with_positions(ball_radius, ball_location, blue_team_positions, red_team_positions),
            blue_team_subscribers,
            red_team_subscribers,
            name,
            update_rate
        }
    }
}

impl<'a> Node for FieldNode<'a> {
    fn name(&self) -> String { String::from(self.name) }

    fn get_update_rate(&self) -> u128 { self.update_rate }

    fn start(&mut self) { }

    fn update(&mut self) {
        self.field.update(self.update_rate);

        for blue_subscriber in self.blue_team_subscribers.iter_mut() {
            blue_subscriber.update_data();
        }

        for red_subscriber in self.red_team_subscribers.iter_mut() {
            red_subscriber.update_data();
        }
    }

    fn shutdown(&mut self) {
        
    }

    fn debug(&self) -> String {
        format!(
            "Field Node:\n{}",
            self.name()
        )
    }
}