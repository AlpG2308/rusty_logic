use std::collections::HashMap;
use crate::model::Model;
use super::gate::{get_state};

#[derive(Debug,Clone)]
pub struct Node{
    id: usize,
    name: String,
    inputs: Vec<usize>,
    gate: Gate,
    output: bool, 
}


#[derive(Debug,Clone)]
pub struct Network {
    Nodes: Vec<Gate>,
    name_id: HashMap<String,usize>,
}


impl Network {
    pub fn create () -> Self{
        Self{Nodes: !vec[Gate],name_id:HashMap::new()}
    }
}
