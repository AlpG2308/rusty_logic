
#[derive(Clone,Debug)]
enum Gate {
    And,
    Or,
    Not,
    Nand,
    Nor,
}

#[derive(Debug,Clone)]
struct Node{
    id: usize,
    name: String,
    inputs: Vec<usize>,
    gate: Gate,
    output: bool, 
}


#[derive(Debug,Clone)]
struct Network {
    nodes: Vec<Gate>
}

fn main() {
    println!("Hello, world!");
}
