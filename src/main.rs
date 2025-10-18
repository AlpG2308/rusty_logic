mod model;
mod simulation;
mod logic;

use std::io::{self, Write};
use simulation::Simulation;
use logic::{Network,Gate};


fn main() {
    let mut circuit = Network::create();

    println!("Logic Network Simulator");
    println!("Commands: add_gate, connect, set_input, simulate, show, exit\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let args: Vec<&str> = input.trim().split_whitespace().collect();
        if args.is_empty() { continue; }

        match args[0] {
            "add_gate" => {
                if args.len() < 3 { println!("Usage: name ,type\n Types: and or not nand nor input"); continue; }
                let g_type = match args[2] {
                    "and" => Gate::And,
                    "or" => Gate::Or,
                    "not" => Gate::Not,
                    "nand" => Gate::Nand,
                    "nor" => Gate::Nor,
                    "input" => Gate::Input,
                    _ => { println!("Unknown gate type"); continue; }
                };
                circuit.add_node(args[1].to_string(), g_type);
                println!("Added gate {}", args[1]);
            }

            "connect" => {
                if args.len() < 3 { println!("Usage: connect <src> <dst>"); continue; }
                circuit.connect(args[1], args[2]);
            }

            "set_input" => {
                if args.len() < 3 { println!("Usage: set_input <gate> <true/false>"); continue; }
                let val = args[2].parse::<bool>().unwrap_or(false);
                circuit.set_input(args[1], val);
            }

            "simulate" => {
                let steps = args.get(1).and_then(|x| x.parse::<f64>().ok()).unwrap_or(1.0);
                let mut sim = Simulation { model: circuit.clone(), t: 0.0, t_end: steps, dt: 1.0 };
                sim.run();
            }

            "list" => circuit.show_nodes(),

            "exit" => break,

            _ => println!("Unknown command"),
        }
    }
}