// not my actual day one a, just a modified part b
#[aoc(day1, part1)]
fn day01part1(input: &str) -> i32 {
    let mut sum = 0;
    for line2 in input.lines() {
        let mut first: i32 = -1;
        let mut last: i32 = -1;

        let line = line2.to_string();

        // let digits: Vec<i32> = Vec::new();

        for i in 0..line.len() {
            let slice = &line[i..];

            let d = if slice.starts_with("1") {
                1
            } else if slice.starts_with("2") {
                2
            } else if slice.starts_with("3") {
                3
            } else if slice.starts_with("4") {
                4
            } else if slice.starts_with("5") {
                5
            } else if slice.starts_with("6") {
                6
            } else if slice.starts_with("7") {
                7
            } else if slice.starts_with("8") {
                8
            } else if slice.starts_with("9") {
                9
            } else {
                10
            };

            if d != 10 {
                if first == -1 {
                    first = d as i32;
                }
                last = d as i32;
            }
        }
        let cal_val = first * 10 + last;
        sum += cal_val;
    }
    return sum;
}
