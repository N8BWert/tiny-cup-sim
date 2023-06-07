use ncomm::node::Node;

/// The Sim Camera Node takes the simulated robot, ball, and field points and creates an image-like
/// representation (as similar to that of the field camera as possible) and sends it out.
/// 
/// The Sim Camera Node is meant to simulate the field camera as close as possible
pub struct SimCameraNode<'a> {
    name: &'a str,
    update_rate: u128,
}

impl<'a> SimCameraNode<'a> {
    pub fn new(name: &'a str, update_rate: u128) -> Self {
        Self {
            name,
            update_rate
        }
    }
}

impl<'a> Node for SimCameraNode<'a> {
    fn name(&self) -> String { String::from(self.name) }

    fn get_update_rate(&self) -> u128 { self.update_rate }

    fn start(&mut self) {
        
    }

    fn update(&mut self) {
        
    }

    fn shutdown(&mut self) {
        
    }

    fn debug(&self) -> String {
        format!(
            "Simulation Camera Node:\n{}",
            self.name()
        )
    }
}