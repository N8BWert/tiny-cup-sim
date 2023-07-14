use ndarray::Array1;

pub trait Dynamic {
    fn apply_local_linear_velocity(&mut self, _linear_velocity: Array1<f32>) {}

    fn apply_local_angular_velocity(&mut self, _angular_velocity: f32) {}

    fn apply_local_velocity(&mut self, linear_velocity: Array1<f32>, angular_velocity: f32) {
        self.apply_local_linear_velocity(linear_velocity);
        self.apply_local_angular_velocity(angular_velocity);
    }

    /// Update rate is in terms of milliseconds
    fn update(&mut self, update_rate: u128);
}