use std::collections::HashMap;

pub fn day3() {
    let d3p1 = include_str!("../data/day3p1.txt");
    let d3p2 = include_str!("../data/day3p2.txt");
    println!("Day 2 Part 1: {}", sum_all_line_part_numbers(d3p1));
    println!("Day 2 Part 2: {}", sum_of_gears(d3p2));
}

fn sum_of_gears(input: &str) -> i32 {
    let mut sum = 0;
    let mut top: Option<&str> = None;
    let mut bottom: Option<&str> = None;
    let mut row = 0;
    // hashmap of (str, gear_index) -> vec<i32>
    let mut gears: HashMap<(String, usize), Vec<i32>> = HashMap::new();
    for line in input.lines() {
        if row == 0 {
            top = None;
        } else {
            top = Some(input.lines().nth(row - 1).unwrap());
        }
        if row == input.lines().count() - 1 {
            bottom = None;
        } else {
            bottom = Some(input.lines().nth(row + 1).unwrap());
        }
        gear_search(&mut gears, line, top, bottom);
        row += 1;
    }
    // sum gears with vec.len() == 2
    for gear in gears {
        if gear.1.len() == 2 {
            sum += gear.1[0] * gear.1[1];
        }
    }
    return sum;
}

fn gear_search(gears: &mut HashMap<(String, usize), Vec<i32>>, row: &str, top: Option<&str>, bottom: Option<&str>) {
    let regex = regex::Regex::new(r"[0-9]+").unwrap();
    // match the row with the regex
    let mut row_match = regex.find_iter(row);
    // iterate over the matches
    for m in row_match {
        // get the number
        let number = m.as_str().parse::<i32>().unwrap();
        // get the index
        let index = m.start();
        let len = m.end() - m.start();
        update_touch_info(gears, Some(row), index, len, number);
        update_touch_info(gears, top, index, len, number);
        update_touch_info(gears, bottom, index, len, number);
    }
}

fn update_touch_info(gears: &mut HashMap<(String, usize), Vec<i32>>, line: Option<&str>, index: usize, len: usize, number: i32) {
        if let Some(line) = line {
        // iter over the line chars starting at index - 1 and ending at index + len + 1
        let start = if index == 0 { 0 } else { index - 1 };
        let end = if index + len + 1 > line.len() {
            line.len()
        } else {
            index + len + 1
        };
        // same but with char indices
        let mut char_index = 0;
        for c in line.chars() {
            if char_index >= start && char_index < end {
                // if char is '*' its a gear
                if c == '*' {
                    // if the gear is not in the hashmap
                    if !gears.contains_key(&(line.to_string(), char_index)) {
                        // add the gear to the hashmap
                        gears.insert((line.to_string(), char_index), vec![number]);
                    }
                    // if the gear is in the hashmap
                    else {
                        // add the number to the gear
                        gears.get_mut(&(line.to_string(), char_index)).unwrap().push(number);
                    }
                }
            }
            char_index += 1;
        }
    }
}

fn sum_all_line_part_numbers(input: &str) -> i32 {
    let mut sum = 0;
    let mut top: Option<&str> = None;
    let mut bottom: Option<&str> = None;
    let mut row = 0;
    for line in input.lines() {
        if row == 0 {
            top = None;
        } else {
            top = Some(input.lines().nth(row - 1).unwrap());
        }
        if row == input.lines().count() - 1 {
            bottom = None;
        } else {
            bottom = Some(input.lines().nth(row + 1).unwrap());
        }
        sum += part_number_sum_in_row(line, top, bottom);
        row += 1;
    }
    return sum;
}

fn part_number_sum_in_row(row: &str, top: Option<&str>, bottom: Option<&str>) -> i32 {
    let mut sum = 0;
    let regex = regex::Regex::new(r"[0-9]+").unwrap();
    // match the row with the regex
    let mut row_match = regex.find_iter(row);
    // iterate over the matches
    for m in row_match {
        // get the number
        let number = m.as_str().parse::<i32>().unwrap();
        // get the index
        let index = m.start();
        let len = m.end() - m.start();
        if (touches_line(Some(row), index, len)
            || touches_line(top, index, len)
            || touches_line(bottom, index, len))
        {
            sum += number;
        }
    }
    return sum;
}


fn touches_line(line: Option<&str>, index: usize, len: usize) -> bool {
    let mut res = false;
    if let Some(line) = line {
        // iter over the line chars starting at index - 1 and ending at index + len + 1
        let start = if index == 0 { 0 } else { index - 1 };
        let end = if index + len + 1 > line.len() {
            line.len()
        } else {
            index + len + 1
        };
        for c in line.chars().skip(start).take(end - start) {
            // if char is not a digit or a dot
            if !c.is_digit(10) && c != '.' {
                res = true;
                break;
            }
        }
    } else {
        res = false;
    }
    return res;
}
