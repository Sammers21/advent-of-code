pub fn day2() {
    let d2p1 = include_str!("../data/day2p1.txt");
    let d2p2 = include_str!("../data/day2p2.txt");
    println!("Day 2 Part 1: {}", cubes_ids_sum_part1(d2p1));
    println!("Day 2 Part 2: {}", cubes_sum_of_pws_part2(d2p2));
}

fn cubes_sum_of_pws_part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| game_power(&line))
        .sum()
}

fn cubes_ids_sum_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| is_game_possible(&line))
        .filter(|game| game.1)
        .map(|game| game.0)
        .sum()
}

fn game_power(input: &str) -> i32 {
    let mut red = -1;
    let mut green = -1;
    let mut blue = -1;
    // split input by ':'
    let mut game_split = input.split(':');
    let _ = game_split.next().unwrap();
    let game = game_split.next().unwrap();
    // split input by ';'
    let rounds = game.split(';');
    // for each round calculate the sum of cubes
    for round in rounds {
        // split round by ','
        // and trim each cube and get a vector of tuples (number, color)
        let cubes = round.split(',')
            .map(|cube| cube.trim())
            .map(|cube| cube.split(' '))
            .map(|cube| {
                let mut cube = cube;
                let number = cube.next().unwrap().parse::<i32>().unwrap();
                let color = cube.next().unwrap().trim();
                // println!("{} {}", number, color);
                (number, color)
            });
        // group cubes by color
        // group cubes by color
        let mut red_cubes = 0;
        let mut green_cubes = 0;
        let mut blue_cubes = 0;
        for cube in cubes {
            match cube.1 {
                "red" => red_cubes += cube.0,
                "green" => green_cubes += cube.0,
                "blue" => blue_cubes += cube.0,
                col => panic!("Invalid color {}", col),
            }
        }
        if red == -1 {
            red = red_cubes;
        } else if red < red_cubes {
            red = red_cubes;
        } 
        if green == -1 {
            green = green_cubes;
        } else if green < green_cubes {
            green = green_cubes;
        }
        if blue == -1 {
            blue = blue_cubes;
        } else if blue < blue_cubes {
            blue = blue_cubes;
        }
    }
    return  red * green * blue;
}

fn is_game_possible(input: &str) -> (i32, bool) {
    let red = 12;
    let green = 13;
    let blue = 14;
    // split input by ':'
    let mut game_split = input.split(':');
    let game_info = game_split.next().unwrap();
    let mut split = game_info.split(' ');
    split.next();
    let game_number = split.next().unwrap().parse::<i32>().unwrap();
    let game = game_split.next().unwrap();
    // split input by ';'
    let rounds = game.split(';');
    // for each round calculate the sum of cubes
    for round in rounds {
        // split round by ','
        // and trim each cube and get a vector of tuples (number, color)
        let cubes = round.split(',')
            .map(|cube| cube.trim())
            .map(|cube| cube.split(' '))
            .map(|cube| {
                let mut cube = cube;
                let number = cube.next().unwrap().parse::<i32>().unwrap();
                let color = cube.next().unwrap();
                (number, color)
            });
        // group cubes by color
        let mut red_cubes = 0;
        let mut green_cubes = 0;
        let mut blue_cubes = 0;
        for cube in cubes {
            match cube.1 {
                "red" => red_cubes += cube.0,
                "green" => green_cubes += cube.0,
                "blue" => blue_cubes += cube.0,
                _ => panic!("Invalid color"),
            }
        }
        if red_cubes > red || green_cubes > green || blue_cubes > blue {
            return (game_number, false);
        }
    }
    return (game_number, true);
}

