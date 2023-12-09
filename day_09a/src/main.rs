use std::fs;

use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    // let input = "0 3 6 9 12 15
    // 1 3 6 10 15 21
    // 10 13 16 21 30 45";

    let lines = input.lines();

    let mut ans = 0;
    for line in lines {
        let line = line.trim();
        if line == "" {
            continue;
        }

        let mut nums: Vec<i64> = line.split(" ").map(|num| num.parse().unwrap()).collect();

        let mut triangle = Vec::new();
        triangle.push(nums);

        let mut found_non_zero = true;
        while found_non_zero {
            let mut new_row = Vec::new();
            let old_row = &triangle[triangle.len() - 1];

            found_non_zero = false;
            for i in 1..(old_row.len()) {
                let v = old_row[i] - old_row[i - 1];
                found_non_zero = found_non_zero || (v != 0);
                new_row.push(v);
            }

            triangle.push(new_row);
        }

        {
            let idx = triangle.len() - 1;
            let last_row = &mut triangle[idx];
            last_row.push(0);
        }

        for i in (0..(triangle.len() - 1)).rev() {
            let prv = triangle[i + 1][triangle[i + 1].len() - 1];
            let crv = triangle[i][triangle[i].len() - 1];

            let v = prv + crv;
            triangle[i].push(v);
        }

        ans += triangle[0][triangle[0].len() - 1];

        println!("{triangle:?}");
    }
    println!("{ans}");
}
