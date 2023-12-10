#[aoc(day9, part2)]
fn day09part2(input: &str) -> i64 {
    let lines = input.lines();

    let mut ans = 0;
    for line in lines {
        let line = line.trim();
        if line == "" {
            continue;
        }

        let nums: Vec<i64> = line.split(" ").map(|num| num.parse().unwrap()).collect();

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
            last_row.insert(0, 0);
        }

        for i in (0..(triangle.len() - 1)).rev() {
            let prv = triangle[i + 1][0];
            let crv = triangle[i][0];

            let v = crv - prv;
            triangle[i].insert(0, v);
        }

        ans += triangle[0][0];
    }
    return ans;
}
