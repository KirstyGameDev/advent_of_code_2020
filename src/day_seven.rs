use std::fs;
use itertools::Itertools;

pub fn day_seven()
{
    // Figure out how many bag combos can hold our one gold bag following a list of rules (input)
    let bag_rules = fs::read_to_string("input_files/input_07.txt").expect("Something went wrong reading the file.");

    let current_bag = String::from("one gold bag");

    let rules = bag_rules.split(".").map(|rule| rule.to_string()).collect::<Vec<String>>();

    let total = find_num_of_suitable_bags(rules);

    println!("Current total {:?}", total);
}

fn find_num_of_suitable_bags(rules : Vec<String>) -> usize
{
    let mut bags_to_check : Vec<String> = Vec::new();
    bags_to_check.push("shiny gold".to_string());

    let mut total : usize = 0;

    // Repeat through all the potential bags that we may have in order
    while bags_to_check.len() > 0
    {
        let current = bags_to_check.pop().expect("Unable to pop off the end");

        for rule in &rules
        {
            // This is only getting the first part after contain
            // TODO get the whole string after the contains.
            let split = rule.split("contain").collect::<Vec<&str>>();

            println!("{:?}", split[0]);

            if split[0].contains(&current)
            {
                total+=1;
            }
        }
    }

    total

}
