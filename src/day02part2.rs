use std::collections::HashMap;

use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day2, part2)]
fn day02part2(input: &str) -> u32 {
    let mut possible_sum: u32 = 0;

    for line in input.lines() {
        let mut bag_ball_count = HashMap::from([
            ("red".to_string(), 0),
            ("green".to_string(), 0),
            ("blue".to_string(), 0),
        ]);
        let re = Regex::new(r"Game (\d+): (.*)").unwrap();

        let stuff = re.captures(line).unwrap();
        let draws = &stuff[2];

        let re2 = Regex::new(r"[\da-z\s,]*").unwrap();
        for draw in re2.find_iter(draws) {
            let draw = draw.as_str();

            let re3 = Regex::new(r"(\d+) ([a-z]+)").unwrap();
            for ball_count in re3.captures_iter(draw) {
                let count: u32 = (&ball_count[1]).parse().unwrap();
                let color = &ball_count[2];

                bag_ball_count.insert(
                    color.to_string(),
                    count.max(*bag_ball_count.get(color).unwrap()),
                );
            }
        }

        possible_sum += *bag_ball_count.get("red").unwrap()
            * *bag_ball_count.get("blue").unwrap()
            * *bag_ball_count.get("green").unwrap();
    }
    return possible_sum;
}
