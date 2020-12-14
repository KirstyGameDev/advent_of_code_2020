use std::fs;

pub struct Instructions
{
    pub instr : String,
    pub value : i64,
    pub completed : bool
}

pub fn run()
{
    let mut instructions = get_instruction_set();
    let mut total : i64 = 0;
    let mut current :i64 = 0;
    let mut previous_index : usize = 0;

    loop 
    {
        let current_u : usize = current as usize;
        if instructions[current_u].completed
        {
           // We have found our answer to part one.
           println!("Part One answer is : {:?}", total);
           break;
        }
        
        instructions[current_u].completed = true;
        let line = &instructions[current_u];
        let instr = &line.instr;
        let value = &line.value;

    
        if instr.contains("acc")
        {
            // Change the total by the value
            total += value;
            current += 1;
        }
        else if instr.contains("jmp")
        {
            // Jump to the next set of instructions 
            current += value ; 
        }
        else if instr.contains("nop")
        {
            // Move to the next instruction
            current += 1;
        }   
        
        previous_index = current_u;
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
        let instruc : Instructions = Instructions{instr : iter.next().unwrap().to_string(), value : iter.next().unwrap().to_string().parse().unwrap_or_default(), completed : false};
        instructions_vec.push(instruc);
    }

    instructions_vec
    
}