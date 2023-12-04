use std::{collections::HashMap, fs};

use regex::Regex;

fn is_digit(c: u8) -> bool {
    return b'0' <= c && c <= b'9';
}

fn is_symbol(c: u8) -> bool {
    return c != b'.' && !is_digit(c);
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    //     let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let mut sum = 0;
    for line in input.lines() {
        let line = line.trim();
        if line == "" {
            continue;
        }

        let nums = line.split(':').collect::<Vec<&str>>()[1];
        let winning_numbers = nums.split('|').collect::<Vec<&str>>()[0];
        let my_numbers = nums.split('|').collect::<Vec<&str>>()[1];

        println!("{winning_numbers} {my_numbers}");

        let re = Regex::new(r"\d+").unwrap();
        let winning_numbers_i: Vec<u128> = re
            .captures_iter(winning_numbers)
            .map(|n| n.get(0).unwrap().as_str().parse().unwrap())
            .collect();
        let my_numbers_i: Vec<u128> = re
            .captures_iter(my_numbers)
            .map(|n| n.get(0).unwrap().as_str().parse().unwrap())
            .collect();

        println!("{winning_numbers_i:?} {my_numbers_i:?}");

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
    println!("{sum}");
}
