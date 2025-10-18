pub trait Model {
    fn step(&mut self, dt:f64);
    fn print_state(&self);
}