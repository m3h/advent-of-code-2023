use regex::Regex;

#[aoc(day6, part1)]
fn day06part1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();

    let re = Regex::new(r"(\d+)").unwrap();
    let record_times: Vec<i32> = re
        .captures_iter(lines[0])
        .map(|c| c.get(0).unwrap().as_str().parse().unwrap())
        .collect();
    let record_distances: Vec<i32> = re
        .captures_iter(lines[1])
        .map(|c| c.get(0).unwrap().as_str().parse().unwrap())
        .collect();

    let mut ans = 1;
    for race_idx in 0..record_times.len() {
        let max_time = record_times[race_idx];
        let record_distance = record_distances[race_idx];

        let mut win_count = 0;
        for hold_ms in 0..(max_time + 1) {
            let rate = hold_ms;
            let remaining_time = max_time - hold_ms;
            let achieved_distance = rate * remaining_time;

            if achieved_distance > record_distance {
                win_count += 1;
            }
        }
        ans *= win_count;
    }
    return ans;
}
