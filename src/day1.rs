pub fn day1() {
    let d1p1 = include_str!("../data/day1p1.txt");
    let d1p2 = include_str!("../data/day1p2.txt");
    println!("Day 1 Part 1: {}", calibration_of_document_part1(d1p1));
    println!("Day 1 Part 2: {}", calibration_of_document_part2(d1p2));
    // edge case:
    // println!("eighthree is {}", calibration_number_part2("eighthree"));
}

fn calibration_of_document_part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| calibration_number_part2(&line))
        .sum()
}

fn calibration_of_document_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| calibration_number_part1(&line))
        .sum()
}

fn calibration_number_part1(input: &str) -> i32 {
    let mut first_digit: i32 = -1;
    let mut last_digit: i32 = -1;
    // iterate over each character in the string
    for (i, c) in input.chars().enumerate() {
        // if the character is a digit
        if c.is_digit(10) {
            // convert the character to a digit
            let digit = c.to_digit(10).unwrap() as i32;
            if first_digit == -1 {
                first_digit = digit;
                last_digit = digit;
            } else {
                last_digit = digit;
            }
        }
    }
    return first_digit * 10 + last_digit;
}

fn calibration_number_part2(input: &str) -> i32 {
    let mut first_digit: i32 = -1;
    let mut first_digit_index: i32 = -1;
    let mut last_digit: i32 = -1;
    let mut last_digit_index: i32 = -1;
    // rust ragax that match 'one' or 'two' or 'three' or 'four' or 'five' or 'six' or 'seven' or 'eight' or 'nine' or 'zero' as a group
    // or 1 or 2 or 3 or 4 or 5 or 6 or 7 or 8 or 9 or 0
    // same but each case in a separated ragax
    let rgx_list = [
        regex::Regex::new(r"one").unwrap(),
        regex::Regex::new(r"two").unwrap(),
        regex::Regex::new(r"three").unwrap(),
        regex::Regex::new(r"four").unwrap(),
        regex::Regex::new(r"five").unwrap(),
        regex::Regex::new(r"six").unwrap(),
        regex::Regex::new(r"seven").unwrap(),
        regex::Regex::new(r"eight").unwrap(),
        regex::Regex::new(r"nine").unwrap(),
        regex::Regex::new(r"zero").unwrap(),
        regex::Regex::new(r"[0-9]").unwrap(),
    ];
    for rgx in rgx_list.iter() {
        // iterate over matches
        for cap in rgx.captures_iter(input) {
            // iterate over each capture group
            // println!("cap: {:?}", cap);
            cap.iter().for_each(|c| {
                // iterate over each character in the capture group
                // check
                let matched_str = c.unwrap().as_str();
                // chech if the matched string is a digit
                let digit: i32;
                if matched_str.chars().nth(0).unwrap().is_digit(10) {
                    // convert the character to a digit
                    digit = matched_str.chars().nth(0).unwrap().to_digit(10).unwrap() as i32;
                } else {
                    // convert the string to a digit
                    digit = str_to_int(matched_str);
                }
                if first_digit == -1 && first_digit_index == -1 {
                    first_digit = digit;
                    first_digit_index = c.unwrap().start() as i32;
                    last_digit = digit;
                    last_digit_index = c.unwrap().end() as i32;
                } else if first_digit_index > c.unwrap().start() as i32 {
                    first_digit = digit;
                    first_digit_index = c.unwrap().start() as i32;
                } else if last_digit_index < c.unwrap().end() as i32 {
                    last_digit = digit;
                    last_digit_index = c.unwrap().end() as i32;
                }
            });
        }
    }
    return first_digit * 10 + last_digit;
}

fn str_to_int(input: &str) -> i32 {
    return match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "zero" => 0,
        _ => -1,
    };
}
