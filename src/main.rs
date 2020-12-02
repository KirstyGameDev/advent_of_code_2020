use std::fs;
use std::num;

fn main() {

//    day_one();

    day_two();
}


fn day_two()
{
    println!("Starting day two");

    let passwords = fs::read_to_string("input_files/input_02.txt").expect("Something went wrong reading the file.");
    let password_lines = passwords.lines();
    let mut valid_password_count = 0;

    for line in password_lines
    {
        let split_string : Vec<&str> = line.split_whitespace().collect();

        //println!("{:?}", split_string);
        // Need to find the index of the '-' as we may have numbers that are double digits.

        let dash_index : usize = split_string[0].find('-').expect("Unable to find '-' in split_string");

        let min_occ : i32 = split_string[0][..dash_index].parse().expect("Unable to parse min from string to i32");
        let max_occ : i32 = split_string[0][dash_index+1..].parse().expect("Unable to parse max from string to i32");

        let needed_char = &split_string[1].to_string()[0..1];

        let potential_password = &split_string[2];

        //Part one
        if get_valid_char_occurances(min_occ, max_occ, needed_char.to_string(), potential_password)
        {
            valid_password_count +=1;
        }

        // Part Two
        if get_valid_char_placement(min_occ, max_occ, needed_char.to_string(), potential_password)
        {
            valid_password_count +=1;
        }
    }

    println!("Valid password count is {}", valid_password_count);
}

// TODO - tidy up this function from it's brute forced chaos!
fn get_valid_char_placement(min : i32, max :i32, char : String , password : &str) -> bool
{
    let mut valid = true;
    let mut valid_count = 0;
    let mut start_byte : usize = min as usize;
    start_byte -=1;

    let mut finish_byte : usize = max as usize;
    finish_byte -=1;

    let first_char = password.chars().nth(start_byte).unwrap_or_default();
    let second_char = password.chars().nth(finish_byte).unwrap_or_default();

    if char.matches(first_char).count() == 1
    {
        valid_count +=1;
    }
    if char.matches(second_char).count() == 1
    {
        valid_count += 1;
    }

    println!("{:?}", valid_count);

    if valid_count != 1
    {
        valid = false;
    }

    valid
}

fn get_valid_char_occurances(min : i32, max :i32, char : String , password : &str) -> bool
{
    let mut is_valid = false;

    let char_count : usize = password.matches(&char).count();

    if char_count >= min as usize && char_count <= max as usize
    {
        is_valid = true;
    }

    is_valid
}

fn get_input() -> Vec<i32>
{
    let str_contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file.");

    let lines = str_contents.lines();

    let mut number_vec : Vec<i32> = Vec::new();

    for line in lines
    {
        // println!("{}", line);

        let temp : i32 = line.trim().parse().expect("Unable to parse to i32");

        number_vec.push(temp);
    }

    // Return the vec of numbers to the calling function.
    number_vec
}

fn day_one()
{
    println!("Starting day one");

    let input : Vec<i32> = get_input();
    let input_copy = input.to_owned().clone();

    let mut found_numbers : bool = false;
    let mut answer : i32 = 1003971 ;

    // part 1 solution
    /*for num in input
    {
        for second_num in &input_copy
        {
            if num == *second_num
            {
                continue;
            }
            if num + *second_num == 2020
            {
                found_numbers = true;
                answer = multiply_numbers(num, *second_num);
                break;
            }
        }
        if found_numbers
        {
            break;
        }
    }

    println!("The answer for day one is {}", answer);
    */


    // part two solution / tidy up
    let mut part_two_answer : i32 = 0;

    let input_len :usize = input.len();

    for i in 0..input_len
    {
        for j in 1..input_len
        {
            for k in 2..input_len
            {
                found_numbers = equals_desired_number(input[i], input[j], input[k], 2020);

                if found_numbers == true
                {
                    println!("The found numbers were {}, {}, {}", input[i], input[j], input[k]);
                    let temp = multiply_numbers(input[i], input[j]);
                    part_two_answer = multiply_numbers(temp, input[k]);
                    break;
                }
            }
            if found_numbers
            {
                break;
            }
        }

        if found_numbers
        {
            break;
        }
    }

    println!("The answer for part two is {} ", part_two_answer);

}

fn equals_desired_number(x : i32, y :i32, z: i32, answer : i32) -> bool
{
    let mut return_val = false;
    if x+y+z == answer
    {
        return_val = true;
    }

    return_val
}

fn multiply_numbers(x:i32 , y:i32) -> i32
{
    println!("The found numbers for the solution are {}, {}", x , y);
    x * y
}
