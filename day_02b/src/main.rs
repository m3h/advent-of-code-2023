use std::{fs, collections::HashMap};

use regex::Regex;

fn main() {
    let input = fs::read_to_string("input")
    .expect("Error reading file");

    // let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let bag_ball_count = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let mut possible_sum: u32 = 0;

    for line in input.lines() {

        let mut bag_ball_count = HashMap::from([
            ("red".to_string(), 0),
            ("green".to_string(), 0),
            ("blue".to_string(), 0),
        ]);
        let mut game_possible = true;
        println!("{line}");
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

                bag_ball_count.insert(color.to_string(), count.max(*bag_ball_count.get(color).unwrap()));
            }
        }

        possible_sum += *bag_ball_count.get("red").unwrap() * *bag_ball_count.get("blue").unwrap() * *bag_ball_count.get("green").unwrap();
    }
    println!("{possible_sum}");
}
