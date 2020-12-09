use std::fs;

pub struct Instructions
{
    pub instr : String,
    pub value : usize,
    pub completed : bool
}

pub fn run()
{
    let mut instructions = get_instruction_set();
    let mut total : usize = 0;
    let mut current :usize = 0;

    loop 
    {
        let line = &instructions[current];
        let instr = &line.instr;
        let value = &line.value;

        instructions[current].completed = true;

        if instr.contains("acc")
        {
            // Change the total by the value
            println!("matched with acc");
            total += value;
            current += 1;
        }
        else if instr.contains("jmp")
        {
            // Jump to the next set of instructions 
            println!("matched with jmp");
            current += value; 
        }
        else if instr.contains("nop")
        {
            // Move to the next instruction
            println!("matched with nop");
            current += 1;
        }

        
    
    }


}

fn get_instruction_set() -> Vec<Instructions>
{
    let program = fs::read_to_string("input_files/input_08.txt").expect("Something went wrong reading the file.");
    let collection : Vec<&str> = program.split('\n').collect();

    let mut instructions_vec : Vec<Instructions> = Vec::new();

    for value in collection
    {
        let mut iter = value.trim().split_whitespace();
        let temp_tuple : (String, usize, bool) = (iter.next().unwrap().to_string(), iter.next().unwrap().to_string().parse().unwrap_or_default(), false);
        
        let instruc : Instructions = Instructions{instr : temp_tuple.0, value : temp_tuple.1, completed : temp_tuple.2};
        instructions_vec.push(instruc);

    }

    instructions_vec
    
}



