use regex::Regex;

#[aoc(day4, part1)]
fn day04part1(input: &str) -> u128 {
    let mut sum = 0;
    for line in input.lines() {
        let line = line.trim();
        if line == "" {
            continue;
        }

        let nums = line.split(':').collect::<Vec<&str>>()[1];
        let winning_numbers = nums.split('|').collect::<Vec<&str>>()[0];
        let my_numbers = nums.split('|').collect::<Vec<&str>>()[1];

        let re = Regex::new(r"\d+").unwrap();
        let winning_numbers_i: Vec<u128> = re
            .captures_iter(winning_numbers)
            .map(|n| n.get(0).unwrap().as_str().parse().unwrap())
            .collect();
        let my_numbers_i: Vec<u128> = re
            .captures_iter(my_numbers)
            .map(|n| n.get(0).unwrap().as_str().parse().unwrap())
            .collect();

        let mut score: u128 = 0;
        for winning_num in &winning_numbers_i {
            for my_number in &my_numbers_i {
                if winning_num == my_number {
                    if score == 0 {
                        score = 1;
                    } else {
                        score *= 2;
                    }
                }
            }
        }
        sum += score;
    }
    return sum;
}
