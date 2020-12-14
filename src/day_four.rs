use std::fs;

pub fn day_four() {
    println!("Starting day four");

    let passport_data = fs::read_to_string("input_files/input_04.txt")
        .expect("Something went wrong reading the file.");
    //N.B Regex would be useful for solving this issue

    // Part One
    let mut valid_passports: i32 = 0;
    let mut temp : i32 = 0;
    let mut num_of_fields: usize = 0;

    let lines = passport_data.lines();

    let mut optional_field_present: bool = false;
    let mut passport_data : Vec<&str> = Vec::new();

    for line in lines {
        let data: Vec<&str> = line.split_whitespace().collect();

        if data.len() != 0 {
            num_of_fields += data.len();

            // Has this data got cid in here or not?
            for field in data {
                if field.to_string().contains("cid") {
                    optional_field_present = true;
                }
                passport_data.push(field);
            }

            
        } else {
            // Check if the past data was all ok to be valid
            if is_passport_valid(num_of_fields, optional_field_present)
            {
                valid_passports += 1;
             
                let mut is_fields_valid : bool = true;
                
                for field in &passport_data
                {
                    let split : Vec<&str> = field.split(":").collect();

                    if split.len() == 2
                    {
                       if !check_field(split[0].to_string(), split[1].to_string())
                       {
                           is_fields_valid = false;
                       }
                    }
                    
                }

                if is_fields_valid 
                {
                    temp += 1;
                }
               
            }

            
            passport_data.clear();

            num_of_fields = 0;
            optional_field_present = false;
        }
    }

    // Check that we have processsed the last passport properly.
    if num_of_fields != 0 && is_passport_valid(num_of_fields, optional_field_present) 
    {
        valid_passports += 1;

        let mut is_valid = true;
        for field in &passport_data
        {
            let split : Vec<&str> = field.split(":").collect();

            if split.len() == 2
            {
               if !check_field(split[0].to_string(), split[1].to_string())
               {
                  is_valid = false;
               }
            }
        
        }
        if is_valid
        {
            temp+= 1;
        }
    }

    println!("Valid passports in part one : {:?}", valid_passports);
    println!("Valid passports in part two : {:?}", temp);
}

fn is_passport_valid(num_of_fields: usize, found_optional_field: bool) -> bool {
    let mut is_valid: bool = false;
    if num_of_fields == 8 || (num_of_fields == 7 && !found_optional_field) {
        is_valid = true;
    }
    is_valid
}

fn check_field(field : String, value : String) -> bool
{
    let mut valid = false;
    
    // Always valid if it's there or not.
    if field.contains("cid")
    {
        valid = true;
    }

   if field.contains("byr")
   {
        let temp : i32 = value.parse().unwrap();

        if temp >= 1920 && temp <= 2002
        {
            valid = true;
        }
   }
   else if field.contains("iyr")
   {
        let temp : i32 = value.parse().unwrap();

        if temp >= 2010 && temp <= 2020 
        {
            valid = true;
        }
   }
   else if field.contains("eyr")
   {
        let temp : i32 = value.parse().unwrap();

        if temp >= 2020 && temp <= 2030 
        {
            valid = true;
        }
   }
   else if field.contains("hgt")
   {
       let temp : Vec<&str> = value.split(|c| c == 'c'  || c == 'i').collect();
        
       if temp.len() == 2
       {
            let height : i32 = temp[0].parse().unwrap();

            if value.contains("cm") 
            {
                if height >= 150 && height <= 193
                {
                    valid = true;
                }
            }
            else
            {
                if height >= 59 && height <=76
                {
                    valid = true;
                }
            }
       }      
    }
    else if field.contains("hcl")
    {
        let mut temp = value;

        if temp.remove(0) == '#'
        {
            if temp.chars().all(|x| x.is_numeric() || x == 'a' || x == 'b' || x == 'c' || x == 'd' || x == 'e' || x == 'f')
            {
                valid = true;
            }
        }                
    }
    else if field.contains("ecl")
    {
        if value.contains("amb") || value.contains("blu") || value.contains("brn") || value.contains("gry") 
        || value.contains("grn") || value.contains("hzl") || value.contains("oth")
        {
            valid = true;
        }
    }
    else if field.contains("pid")
    {
        println!("{:?}", value);

        if value.len() == 9
        {
            valid = true;            
        }
    }
    
   valid
}

