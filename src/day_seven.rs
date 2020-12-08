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
    let mut bags_checked : Vec<String> = Vec::new();

    let mut total : usize = 0;

    // Repeat through all the potential bags that we may have in order
    while bags_to_check.len() > 0
    {
        let current = bags_to_check.pop().expect("Unable to pop off the end");
        for rule in &rules
        {
            match rule.find(&current)
            {
                Some(s) =>
                {
                    // Assuming here that if we're past the current bags string length, that it's not the first part
                    // of the rule e.g. "Shiny Gold Bags contains ..."
                    if s > current.len()
                    {

                        // We have a new bag to check in the rules so let's add it to the Vec
                        let r : Vec<_> = rule.split_whitespace().collect();
                        let mut new_bag = r[0].to_string();
                        new_bag.push_str(" ");
                        new_bag.push_str(&r[1].to_string());

                        if new_bag != current && !bags_to_check.contains(&new_bag) && !bags_checked.contains(&new_bag)
                        {
                            bags_to_check.push(new_bag);

                            total += 1;
                        }

                    }
                }
                None => {continue;}
            }
        }
        // Now that we've checked over this bag, lets add this to the checked pile.
        bags_checked.push(current);
    }

    total

}
