use std::collections::HashMap;
use crate::model::Model;
use super::gate::Gate;

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
    Nodes: Vec<Node>,
    name_id: HashMap<String,usize>,
}


impl Network {
    //initalize network
    pub fn create () -> Self{
        Self{Nodes: vec![],name_id:HashMap::new()}
    }
    //add an intermediate node to the network
    pub fn add_node(&mut self, name:String,gate_type:Gate){
        let id = self.Nodes.len();
        self.name_id.insert(name.to_string(),id);
        self.Nodes.push(Node{
            id:id,
            name:name.to_string(),
            gate:gate_type,
            inputs:vec![],
            output:false,
        });
    }
    // show all nodes and ids in the network
    pub fn show_nodes(&self){
        for node in self.name_id.iter(){
            println!("Name: {}\tID: {}",node.0,node.1);
        }
    }
    // connect two nodes in the network
    pub fn connect(&mut self, from :&str, to:&str){
        if let (Some(&from_id),Some(&to_id)) = (self.name_id.get(from),self.name_id.get(to))
        {
            self.Nodes[to_id].inputs.push(from_id);
        }
        else{
            println!("Gate not found Source: {} Target: {}",from,to);
            self.show_nodes();
        }

    }
    pub fn set_input(&mut self, name:&str,state:bool){
        if let Some(&input_id) = self.name_id.get(name){
            self.Nodes[input_id].output = state;
        }
        else{
            println!("Gate not found: {} ",name);
            self.show_nodes();            
        }
    }
}
impl Model for Network {
    //get all current outputs
    // update all nodes based on gate type and input
    fn step(&mut self, dt:f64){
        let current_states = self.Nodes.iter().map(|x| x.output).collect::<Vec<bool>>();
        for node in self.Nodes.iter_mut(){
            if let Gate::Input = node.gate{
                continue;
            }
            let input_states = node.inputs.iter().map(|&i| current_states[i]).collect::<Vec<bool>>();
            node.output = node.gate.get_state(&input_states);
        }
    }
    fn print_state(&self){
        for node in self.Nodes.iter(){
            println!("Node {} (Gate: {:?}) Output: {}",node.name,node.gate,node.output);
        }
    }
}   