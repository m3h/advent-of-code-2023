use std::fs;

use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    // let input = "LLR

    // AAA = (BBB, BBB)
    // BBB = (AAA, ZZZ)
    // ZZZ = (ZZZ, ZZZ)";

    let lines: Vec<&str> = input.lines().collect();

    let instructions = lines[0].as_bytes();

    let mut map = HashMap::new();

    for line in &lines[1..] {
        let line = line.trim();
        if line == "" {
            continue;
        }

        let line = line
            .replace("=", "")
            .replace("(", "")
            .replace(")", "")
            .replace(",", "")
            .replace("  ", " ");

        let line_parts: Vec<_> = line.split(" ").collect();
        let src = line_parts[0];
        let dst_l = line_parts[1];
        let dst_r = line_parts[2];

        map.insert(src.to_string(), (dst_l.to_string(), dst_r.to_string()));
    }

    let mut loc = "AAA".to_string();
    let mut step_count = 0;
    let mut instruction_idx = 0;
    while loc != "ZZZ" {
        step_count += 1;
        let instruction = instructions[instruction_idx];
        instruction_idx = (instruction_idx + 1) % instructions.len();

        if instruction == b'L' {
            loc = map.get(&loc).unwrap().0.clone();
        } else if instruction == b'R' {
            loc = map.get(&loc).unwrap().1.clone();
        } else {
            panic!("oh no! {instruction}");
        }
    }
    println!("{step_count}");
}
