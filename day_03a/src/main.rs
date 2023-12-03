use std::{collections::HashMap, fs};

use regex::Regex;

fn is_digit(c: u8) -> bool {
    return b'0' <= c && c <= b'9';
}

fn is_symbol(c: u8) -> bool {
    return c != b'.' && !is_digit(c);
}

fn process_numbers(schematic: &mut Vec<Vec<u8>>) -> i32 {
    let mut sum = 0;

    for i in 0..schematic.len() {
        for j in 0..schematic[i].len() {
            if is_digit(schematic[i][j]) {
                let mut k = j;
                while k < schematic[i].len() && is_digit(schematic[i][k]) {
                    k += 1;
                }
                if k >= schematic[i].len() || !is_digit(schematic[i][k]) {
                    k -= 1;
                }

                let mut found_symbol = false;
                for l in j..(k + 1) {
                    for x in 0..3 {
                        for y in 0..3 {
                            let xi = (x + i) as i32 - 1;
                            let yi = (y + l) as i32 - 1;

                            found_symbol = found_symbol
                                || xi >= 0
                                    && yi >= 0
                                    && xi < schematic.len() as i32
                                    && yi < schematic[xi as usize].len() as i32
                                    && is_symbol(schematic[xi as usize][yi as usize])
                        }
                    }
                }
                if found_symbol {
                    let mut num = 0;
                    let mut multiplier = 1;
                    for x in (j..(k + 1)).rev() {
                        num += (schematic[i][x] - b'0') as i32 * multiplier;
                        multiplier *= 10;

                        schematic[i][x] = b'.';
                    }
                    sum += num;
                }
            }
        }
    }

    return sum;
}

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    // let input = "467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..";

    let mut schematic: Vec<Vec<u8>> = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        schematic.push(Vec::new());
        for c in line.as_bytes() {
            let idx = schematic.len() - 1;
            schematic[idx].push(*c);
        }
    }

    let sum = process_numbers(&mut schematic);

    println!("{sum}")
}
