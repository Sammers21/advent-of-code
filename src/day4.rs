use std::{collections::HashSet, f32::consts::E};

pub fn day4() {
    let d4p1 = include_str!("../data/day4p1.txt");
    //     let d4p2 = include_str!("../data/day4p2.txt");
    let test = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    println!("TEST {}", sum_of_cards(test));
    println!("Day 4 Part 1 {}", sum_of_cards(d4p1));

}

fn sum_of_cards(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let count_winning = count_winning(line);
            count_winning
        })
        .sum()
}

fn count_winning(input: &str) -> i32 {
    let mut winning_numbers = 0;
    let mut winning_and_all = input.split(":").skip(1).next().unwrap().split("|");
    let winning = winning_and_all.next().unwrap();
    let winning_set =
        HashSet::<String>::from_iter(winning.split(" ").map(|st| st.trim().to_string()).filter(|x| !x.eq("")));
    let all = winning_and_all.next().unwrap();
    let all_set = HashSet::<String>::from_iter(all.split(" ").map(|st| st.trim().to_string()).filter(|x| !x.eq("")));
    for st in all_set.into_iter() {
        if winning_set.contains(&st) {
            winning_numbers += 1;
        }
    }
    // println!("Winning numbers: {}", winning_numbers);
    if winning_numbers == 0 || winning_numbers == 1  {
        return winning_numbers;
    } else {
        return (2 as i32).pow((winning_numbers - 1) as u32);
    }
}
