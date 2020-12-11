use core::str::Lines;
use std::fs;

pub fn day_three() {
    println!("Starting day three");

    let map = fs::read_to_string("input_files/input_03.txt").expect("Something went wrong reading the file.");
    let mut lines = map.lines();
    
    // Part One
    // Find out how many tree's we would encounter with the toboggan by going right 3, down 1, starting
    // from the top left and working our way down.
    let mut lines_copy = lines.clone();
    let part_one_answer: i64 = traverse_slope(&mut lines_copy, 3, 1);
    println!("Part One - {:?}", part_one_answer);

    /*
    Part two
    Checking the rest of the slopes using the same method but this time , using the following ways of traversel.
    Right 1, down 1.
    Right 3, down 1. (This is the slope you already checked in part one)
    Right 5, down 1.
    Right 7, down 1.
    Right 1, down 2.
    */

    // Have to clone lines from here on in
    // This is because ownership has been given to the traverse_slope instance above and main has lost ownership and knowledge of lines.
    
    let mut part_two_answer : i64 = 0;

    let mut answer = traverse_slope(&mut lines.clone(), 1, 1);
    println!("1-1 answer  - {:?}", answer);
    part_two_answer = answer * part_one_answer;

    answer = traverse_slope(&mut lines.clone(), 5, 1);
    println!("5-1 answer  - {:?}", answer);
    part_two_answer *= answer;

    answer = traverse_slope(&mut lines.clone(), 7, 1);
    println!("7-1 answer  - {:?}", answer);
    part_two_answer *= answer;

    answer = traverse_slope(&mut lines.clone(), 1, 2);
    println!("1-2 answer  - {:?}", answer);
    part_two_answer *= answer;

    println!("Part Two Answer is : {:?}", part_two_answer);

}

fn traverse_slope(path: &mut Lines, right: usize, down: usize) -> i64 {
    let tree = String::from('#');

    let mut max_length: usize = 0;
    let mut check_byte: usize = 0;
    let mut trees_encountered: i32 = 0;
    let mut at_start: bool = true;

    let mut current_line: usize = 0;

    for line in path {
        if at_start {
            at_start = false;
            continue;
        }

        current_line +=1;

        // skip over this line if we're not going down by 1.
        if down > 1 && current_line % down != 0
        {
            println!("skipping this line");
            continue;
        }

        
        // Find out our max boundary (removes the assumption that all the input lines are the same length)
        let max_length = line.len();
        check_byte += right;

        if check_byte > max_length - 1 {
            check_byte -= max_length;
        }

        let value = line.chars().nth(check_byte).unwrap_or_default().to_string();

        

        if value == tree {
            trees_encountered += 1;
        }
    }

    trees_encountered as i64
}
