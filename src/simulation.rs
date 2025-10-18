use crate::model::Model;

pub struct Simulation<T:Model>{
    pub model: T,
    pub t: f64,
    pub t_end: f64,
    pub dt: f64,
}
impl<T: Model> Simulation<T>{
    fn update(&mut self){
        self.model.step(self.dt);
        self.t += self.dt;
        self.model.print_state();
    }
    pub fn run(&mut self){
        let steps = (self.t_end/self.dt).ceil() as usize;
        (0..steps).for_each(|_| self.update());
    }
}
