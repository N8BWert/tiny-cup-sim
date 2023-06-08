use ncomm::node::Node;

use crate::sim_field::field_representations::field::Field;

pub struct FieldNode<'a> {
    pub field: Field,
    name: &'a str,
    update_rate: u128,
}

impl<'a> FieldNode<'a> {
    pub fn new(name: &'a str, update_rate: u128, ball_radius: u8) -> Self {
        Self {
            field: Field::new(ball_radius),
            name,
            update_rate
        }
    }

    pub fn new_with_positions(name: &'a str, update_rate: u128,
                              ball_radius: u8, ball_location: (f64, f64),
                              blue_team_positions: Vec<(f64, f64)>,
                              red_team_positions: Vec<(f64, f64)>) -> Self {
        Self {
            field: Field::new_with_positions(ball_radius, ball_location, blue_team_positions, red_team_positions),
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