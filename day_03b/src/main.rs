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
            if schematic[i][j] == b'*' {
                let mut nums = Vec::new();
                for x in 0..3 {
                    for y in 0..3 {
                        let xi = (x + i) as i32 - 1;
                        let yi = (y + j) as i32 - 1;

                        if xi >= 0
                            && yi >= 0
                            && xi < schematic.len() as i32
                            && yi < schematic[xi as usize].len() as i32
                            && is_digit(schematic[xi as usize][yi as usize])
                        {
                            let mut s = yi;
                            let mut e = yi;

                            while s >= 0 && is_digit(schematic[xi as usize][s as usize]) {
                                s -= 1;
                            }
                            if s < 0 || !is_digit(schematic[xi as usize][s as usize]) {
                                s += 1;
                            }

                            while e < schematic[xi as usize].len() as i32 && is_digit(schematic[xi as usize][e as usize]) {
                                e += 1;
                            }
                            if e >= schematic[xi as usize].len() as i32
                                || !is_digit(schematic[xi as usize][e as usize])
                            {
                                e -= 1;
                            }

                            let mut num = 0;
                            let mut multiplier = 1;
                            for x in (s..(e + 1)).rev() {
                                num +=
                                    (schematic[xi as usize][x as usize] - b'0') as i32 * multiplier;
                                multiplier *= 10;

                                schematic[xi as usize][x as usize] = b'.';
                            }
                            nums.push((num, s, e, xi));
                        }
                    }
                }

                if nums.len() == 2 {
                    sum += nums[0].0 * nums[1].0;
                }
                for (num, s, e, xi) in nums {
                    let mut te = e;
                    let mut tnum = num;
                    while tnum > 0 {
                        let d = (tnum % 10) as u8;
                        schematic[xi as usize][te as usize] = d + b'0';
                        tnum /= 10;
                        te -= 1;
                    }
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
