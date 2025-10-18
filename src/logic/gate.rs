// implement gates for logic circuits

#[derive(Clone,Debug)]
pub enum Gate {
    And,
    Or,
    Not,
    Nand,
    Nor,
    Input,
}

//don't point to vector heap location but to elements e.g. not &Vec<bool>
//check for boolean states in connected vectors and react depending on that
//for example "and" go throug inputs only if all true
// input just completness -> make sure we always have a boolean but doesnt effect input state -> is manually set by user
impl Gate {
    pub fn get_state(&self, inputs: &[bool])-> bool {
        match self {
            Gate::And => inputs.iter().all(|&x| x),
            Gate::Or  => inputs.iter().any(|&x| x),
            Gate::Not => !inputs[0],
            Gate::Nand => !inputs.iter().all(|&x| x),
            Gate::Nor => !inputs.iter().any(|&x| x),
            Gate::Input => inputs.get(0).cloned().unwrap_or(false),
        }
    }              
}