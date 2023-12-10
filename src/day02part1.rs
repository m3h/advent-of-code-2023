use std::collections::HashMap;

use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day2, part1)]
fn day02part1(input: &str) -> u32 {
    let bag_ball_count = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut possible_sum: u32 = 0;

    for line in input.lines() {
        let mut game_possible = true;
        let re = Regex::new(r"Game (\d+): (.*)").unwrap();

        let stuff = re.captures(line).unwrap();
        let game_number: u32 = (&stuff[1]).parse().unwrap();
        let draws = &stuff[2];

        let re2 = Regex::new(r"[\da-z\s,]*").unwrap();
        for draw in re2.find_iter(draws) {
            let draw = draw.as_str();

            let re3 = Regex::new(r"(\d+) ([a-z]+)").unwrap();
            for ball_count in re3.captures_iter(draw) {
                let count: u32 = (&ball_count[1]).parse().unwrap();
                let color = &ball_count[2];

                if *bag_ball_count.get(color).unwrap() < count {
                    game_possible = false;
                }
            }
        }

        if game_possible {
            possible_sum += game_number;
        }
    }
    return possible_sum;
}
