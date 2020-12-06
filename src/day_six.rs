use std::fs;
use std::collections::HashSet;


pub fn day_six()
{
    // Part one - Find the unique amount of answers answered yes for each group.

    let groups_answers = fs::read_to_string("input_files/input_06.txt")
        .expect("Something went wrong reading the file.");

    let answers = groups_answers.lines();
    let mut num_of_yes_per_group : Vec<usize> = Vec::new();
    let mut num_of_matching_yes : Vec<usize> = Vec::new();

    let mut collective_answer : String = String::new();

    for answer in answers
    {
        if answer.is_empty()
        {
            let result = check_groups_answers(collective_answer);
            num_of_yes_per_group.push(result);

            collective_answer = String::new();
            continue;
        }

        collective_answer.push_str(answer);
    }

let mut total : usize = 0;
    // Final check
    if !collective_answer.is_empty()
    {
        total = check_groups_answers(collective_answer);
    }
    // Add on everything else
    for num in num_of_yes_per_group
    {
        total += num;
    }



    println!("The answer for part one is {:?}", total);

    // Part two - identify where the group agreed on their answers
}

fn check_groups_answers(collective_answer : String) -> usize
{
    let mut unique_answers = Vec::new();

    println!("{:?}", collective_answer);

    let mut chars = collective_answer.chars();

    loop
    {
        match chars.next()
        {
            Some(letter) =>
            {
                println!( "Checking letter {:?}", letter);

                if !unique_answers.contains(&letter)
                {
                    unique_answers.push(letter);
                }
            }
            None => {break;}
        }
    }

    let answers_count = unique_answers.len();

    println!("Answers count {:?}", answers_count);
    answers_count
}
