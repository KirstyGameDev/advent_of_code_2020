use std::fs;

pub fn day_four() {
    println!("Starting day four");

    let passport_data = fs::read_to_string("input_files/input_04.txt")
        .expect("Something went wrong reading the file.");
    //N.B Regex would be useful for solving this issue

    // Part One
    let optional_field = String::from("cid");

    let mut valid_passports: i32 = 0;
    let mut num_of_fields: usize = 0;

    let lines = passport_data.lines();

    let mut optional_field_present: bool = false;

    for line in lines {
        let data: Vec<&str> = line.split_whitespace().collect();

        if data.len() != 0 {
            num_of_fields += data.len();

            // Has this data got cid in here or not?
            for field in data {
                if field.to_string().contains("cid") {
                    optional_field_present = true;
                }
            }
        } else {
            // CHeck if the past data was all ok to be valid
            if is_passport_valid(num_of_fields, optional_field_present) {
                valid_passports += 1;
            }

            num_of_fields = 0;
            optional_field_present = false;
        }
    }

    // Check that we have processsed the last passport properly.
    if num_of_fields != 0 && is_passport_valid(num_of_fields, optional_field_present) {
        valid_passports += 1;
    }

    println!("Valid passports in part one : {:?}", valid_passports);
}

fn is_passport_valid(num_of_fields: usize, found_optional_field: bool) -> bool {
    let mut is_valid: bool = false;
    if num_of_fields == 8 || (num_of_fields == 7 && !found_optional_field) {
        is_valid = true;
    }
    is_valid
}
