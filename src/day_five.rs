use std::fs;

pub fn day_five()
{
    println!("Starting day five");

    let seat_data = fs::read_to_string("input_files/input_05.txt").expect("Something went wrong reading the file.");

    let lines = seat_data.lines();
    let mut highest_seat :i32 = 0;

    let mut seats_occupied : Vec<i32> = Vec::new();

    for line in lines
    {
        let result = get_seat_info(line);

        if result > highest_seat
        {
            highest_seat = result;
        }

        seats_occupied.push(result);
    }

    println!("Part one - Highest seat is {:?}", highest_seat);

    // Sort the vec.
    seats_occupied.sort();
    let mut last_seat_checked : i32 = seats_occupied[0];
    let num_of_seats = seats_occupied.len();

    for i in 0..num_of_seats-1
    {
        if seats_occupied[i] + 2 == seats_occupied[i+1]
        {
            // Then our seat is found
            println!("Seat found {:?}", seats_occupied[i] + 1);
        }
    }
}

// Returns the seat ID to check
fn get_seat_info(line : &str ) -> i32
{
    // seat info contains row, column, seat num
    let mut seat_info = (0, 0, 0);
    let mut min_row : i32 = 0;
    let mut max_row : i32 = 127;

    let mut min_column : i32 = 0;
    let mut max_column : i32 = 7;

    let mut iter = line.chars();
    let mut result : i32 = 0;

    let mut completed : bool = false;

    let mut char_counter : i32 = 0;
    // Get row
    loop
    {
        match iter.next()
        {
            Some(c) =>
            {
                if (char_counter < 7)
                {
                    let is_lower : bool = c == 'F';
                    result = get_new_limit(is_lower, min_row, max_row);

                    if is_lower
                    {
                        max_row = result;
                        seat_info.0 = min_row;
                    }
                    else
                    {
                        min_row = result;
                        seat_info.0 = max_row;
                    }

                    char_counter+= 1;
                }

                // we need to grab the column
                else
                {
                    let is_lower : bool = c == 'L';
                    result = get_new_limit(is_lower, min_column, max_column);

                    if is_lower
                    {
                        max_column = result;
                        seat_info.1 = min_column;
                    }
                    else
                    {
                        min_column = result;
                        seat_info.1 = max_column;
                    }
                }


            }
            None =>
            {
                completed = true;
            }
        }
        if completed
        {
            break;
        }
    }

    // Get seat number
    seat_info.2 = (seat_info.0 * 8) + seat_info.1;
    //println!("Seat row {:?}, column {:?}, seat ID {:?}.", seat_info.0, seat_info.1, seat_info.2);
    //println!("");

    seat_info.2

}

fn get_new_limit(is_lower: bool, min : i32, max : i32) -> i32
{
    let mut temp : i32 = 0;
    let diff = (max - min)/ 2;

    if diff < 1
    {
        if is_lower
        {
            temp = min ;
        }
        else
        {
            temp = max;
        }
    }
    else
    {
        if is_lower
        {
            temp = min + diff;
        }
        else
        {
            temp = max - diff;
        }
    }

    temp
}
