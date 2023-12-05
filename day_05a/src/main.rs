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
    // let input = "seeds: 79 14 55 13

    // seed-to-soil map:
    // 50 98 2
    // 52 50 48

    // soil-to-fertilizer map:
    // 0 15 37
    // 37 52 2
    // 39 0 15

    // fertilizer-to-water map:
    // 49 53 8
    // 0 11 42
    // 42 0 7
    // 57 7 4

    // water-to-light map:
    // 88 18 7
    // 18 25 70

    // light-to-temperature map:
    // 45 77 23
    // 81 45 19
    // 68 64 13

    // temperature-to-humidity map:
    // 0 69 1
    // 1 0 69

    // humidity-to-location map:
    // 60 56 37
    // 56 93 4

    // ";

    let mut seeds = Vec::new();
    let mut current_src: String = "".to_string();
    let mut current_dst: String = "".to_string();
    let mut maps = Vec::new();
    // let mut maps: HashMap<(String, String), (u128, u128)> = HashMap::new();
    for line in input.lines() {
        let line = line.trim();
        if line == "" {
            continue;
        }

        if line.contains("seeds:") {
            seeds = line
                .split(": ")
                .last()
                .unwrap()
                .split(" ")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u128>>();
        } else if line.contains(":") {
            let types = line.split(" ").collect::<Vec<&str>>()[0];
            let src_type = types.split("-").collect::<Vec<&str>>()[0];
            let dst_type = types.split("-").collect::<Vec<&str>>()[2];

            current_src = src_type.to_string();
            current_dst = dst_type.to_string();

            println!("{src_type} {dst_type}")
        } else {
            let loc_specifier = line
                .split(" ")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u128>>();
            let start_init = loc_specifier[1];
            let dst_init = loc_specifier[0];
            let range_size = loc_specifier[2];

            maps.push((
                current_src.clone(),
                current_dst.clone(),
                start_init,
                dst_init,
                range_size,
            ))
        }
    }

    let mut current_loc_type = "seed".to_string();
    let mut locs = seeds.clone();

    while current_loc_type != "location" {
        // by default, same indices
        let mut new_locs = locs.clone();
        let mut new_loc_type = "".to_string();
        for (src, dst, s, d, range_size) in &maps {
            if *src == current_loc_type {
                new_loc_type = dst.clone();
                for i in 0..locs.len() {
                    if *s <= locs[i] && locs[i] <= *s + *range_size {
                        new_locs[i] = *d + locs[i] - *s;
                    }
                }
            }
        }

        current_loc_type = new_loc_type;
        locs = new_locs;
    }

    println!("{locs:?}");

    let min_loc = locs.iter().min().unwrap();
    println!("{min_loc}");
}
