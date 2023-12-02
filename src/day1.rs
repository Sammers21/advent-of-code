pub fn day1() {
    let input = include_str!("../data/day1.txt");
    println!("Day 1 Part 1: {}", calibration_of_document_part1(input));
}


fn calibration_of_document_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| calibration_number(&line))
        .sum()
}

fn calibration_number(input: &str) -> i32 {
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
    return first_digit*10 + last_digit;
}