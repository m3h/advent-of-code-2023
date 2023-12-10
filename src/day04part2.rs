use regex::Regex;

#[aoc(day4, part2)]
fn day04part2(input: &str) -> i32 {
    let mut scratchcards = Vec::new();
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

        scratchcards.push((winning_numbers_i, my_numbers_i));
    }

    let mut my_cards = Vec::new();
    for _ in 0..scratchcards.len() {
        my_cards.push(1);
    }

    for i in 0..my_cards.len() {
        // count number of winners for this card
        let mut score: u128 = 0;
        for winning_num in &scratchcards[i].0 {
            for my_number in &scratchcards[i].1 {
                if winning_num == my_number {
                    score += 1;
                }
            }
        }

        for j in 0..score {
            my_cards[i + j as usize + 1] += my_cards[i];
        }
    }

    // score
    let mut count = 0;
    for card_count in my_cards {
        count += card_count;
    }
    return count;
}
