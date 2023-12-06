use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    // let input = "Time:      7  15   30
    // Distance:  9  40  200";

    let lines: Vec<String> = input.lines().map(|line| line.replace(" ", "")).collect();

    let max_time: u128 = lines[0].split(":").collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();
    let record_distance: u128 = lines[1].split(":").collect::<Vec<&str>>()[1]
        .parse()
        .unwrap();

    println!("{max_time:?} {record_distance:?}");

    let mut win_count = 0;
    for hold_ms in 0..(max_time + 1) {
        let rate = hold_ms;
        let remaining_time = max_time - hold_ms;
        let achieved_distance = rate * remaining_time;

        if achieved_distance > record_distance {
            win_count += 1;
        }
    }

    println!("{win_count}");
}
