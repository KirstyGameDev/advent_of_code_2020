use std::fs;

fn main() {

    day_one();

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
