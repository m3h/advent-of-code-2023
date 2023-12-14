fn is_horizontal_reflection(map: &Vec<Vec<char>>, y_middle: usize) -> bool {
    if y_middle + 1 > map.len() {
        return false;
    }
    let offset_range = (y_middle + 1).min(map.len() - y_middle - 1);
    if offset_range == 0 {
        return false;
    }
    for offset in 0..(offset_range) {
        for x in 0..map[0].len() {
            if map[y_middle - offset][x] != map[y_middle + offset + 1][x] {
                return false;
            }
        }
    }
    return true;
}

fn is_vertical_reflection(map: &Vec<Vec<char>>, x_middle: usize) -> bool {
    if x_middle + 1 > map[0].len() {
        return false;
    }
    let offset_range = (x_middle + 1).min(map[0].len() - x_middle - 1);
    if offset_range == 0 {
        return false;
    }
    for offset in 0..(offset_range) {
        for y in 0..map.len() {
            if map[y][x_middle - offset] != map[y][x_middle + offset + 1] {
                return false;
            }
        }
    }
    return true;
}

fn find_lines_of_reflection(map: &Vec<Vec<char>>) -> usize {
    let mut ans = 0;
    // horizontal
    for y in 0..map.len() {
        if is_horizontal_reflection(map, y) {
            ans += 100 * (y + 1);
        }
    }

    for x in 0..map[0].len() {
        if is_vertical_reflection(map, x) {
            ans += x + 1;
        }
    }

    return ans;
}

#[aoc(day13, part1)]
fn day13part1(input: &str) -> usize {
    // let input = "#.##..##.
    // ..#.##.#.
    // ##......#
    // ##......#
    // ..#.##.#.
    // ..##..##.
    // #.#.##.#.

    // #...##..#
    // #....#..#
    // ..##..###
    // #####.##.
    // #####.##.
    // ..##..###
    // #....#..#";

    let maps: Vec<Vec<Vec<char>>> = input
        .trim()
        .split("\n\n")
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| line.trim().chars().collect())
                .collect()
        })
        .collect();

    let mut ans = 0;
    for pattern in maps {
        let pattern_ans = find_lines_of_reflection(&pattern);
        ans += pattern_ans;
    }
    return ans;
}
