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
            let seeds_pairs = line
                .split(": ")
                .last()
                .unwrap()
                .split(" ")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u128>>();

            for i in 0..(seeds_pairs.len() / 2) {
                seeds.push((
                    seeds_pairs[i * 2],
                    seeds_pairs[i * 2] + seeds_pairs[i * 2 + 1] - 1,
                ));
            }
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
                start_init + range_size - 1,
                dst_init,
                dst_init + range_size - 1,
            ))
        }
    }

    let mut current_loc_type = "seed".to_string();
    let mut locs = seeds.clone();

    while current_loc_type != "location" {
        // by default, same indices
        let mut new_locs = Vec::new();
        let mut new_loc_type = "".to_string();

        let mut i = 0;
        while i < locs.len() {
            let cur_s = locs[i].0;
            let cur_e = locs[i].1;

            let mut found = false;
            for (src, dst, s_s, s_e, d_s, d_e) in &maps {
                if *src == current_loc_type {
                    new_loc_type = dst.clone();

                    // look for an intersection of ranges

                    let seg_s = cur_s.max(*s_s);
                    let seg_e = cur_e.min(*s_e);

                    if seg_s <= seg_e {
                        found = true;
                        let new_dst_seg_s = seg_s + *d_s - *s_s;
                        let new_dst_seg_e = seg_e + *d_s - *s_s;

                        if seg_s > cur_s {
                            let pre_seg_s = cur_s;
                            let pre_seg_e = seg_s - 1;

                            locs.push((pre_seg_s, pre_seg_e));
                        }
                        if seg_e < cur_e {
                            let post_seg_s = seg_e + 1;
                            let post_seg_e = cur_e;

                            locs.push((post_seg_s, post_seg_e));
                        }
                        new_locs.push((new_dst_seg_s, new_dst_seg_e));
                    }
                }
            }
            if !found {
                new_locs.push((cur_s, cur_e));
            }
            i += 1;
        }

        current_loc_type = new_loc_type;
        locs = new_locs;
    }

    println!("{locs:?}");

    let min_loc = locs.iter().min().unwrap().0;

    println!("{min_loc}");
}
