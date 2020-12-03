use std::fs;
use core::str::Lines;


pub fn day_three()
{
    println!("Starting day three");

    let map = fs::read_to_string("input_files/input_03.txt").expect("Something went wrong reading the file.");
    let lines = map.lines();



    // Part One
    // Find out how many tree's we would encounter with the toboggan by going right 3, down 1, starting
    // from the top left and working our way down.

    println!("Using the method");
    traverse_slope(lines, 3, 1);

    //println!("Part One - Tree's encountered are : {:?}", trees_encountered);

    /*
    Part two
    Checking the rest of the slopes using the same method but this time , using the following ways of traversel.
    Right 1, down 1.
    Right 3, down 1. (This is the slope you already checked in part one)
    Right 5, down 1.
    Right 7, down 1.
    Right 1, down 2.
    */
}

fn traverse_slope(path : Lines<> , right : usize, down : usize)
{
    let tree = String::from('#');

    let mut max_length : usize = 0;
    let mut check_byte : usize = 0;

    let mut trees_encountered : i32 = 0;

    let mut at_start : bool = true;

    for line in path
    {
        if at_start
        {
            at_start = false;
            continue;
        }

        // Find out our max boundary
        let max_length = line.len();

        check_byte += right;


        if check_byte > max_length-1
        {
            println!("reached limit {:?} ", check_byte);

            check_byte -= max_length;
        }

        let value = line.chars().nth(check_byte).unwrap_or_default().to_string();

        println!("check byte / value : {} / {:?}", check_byte, value);

        if value == tree
        {
            trees_encountered += 1;
        }
    }

    println!("Tree's encountered {:?}", trees_encountered);

}
