fn hash(s: &str) -> i32 {
    let mut current_value = 0;
    for c in s.chars() {
        current_value += c as i32;
        current_value *= 17;
        current_value %= 256;
    }

    return current_value;
}

#[aoc(day15, part1)]
fn day15part1(input: &str) -> i32 {
    let input: Vec<&str> = input.trim().split(",").collect();

    let mut hash_sum = 0;
    for word in input {
        let hash_output = hash(word);
        hash_sum += hash_output;
    }
    return hash_sum;
}
