enum ArrangementType {
    Invalid,
    Valid,
    Incomplete,
}

fn consistent_arrangement(visual: &Vec<char>, numeric: &Vec<i32>) -> ArrangementType {
    let mut numeric_idx = 0;
    let mut group_size = 0;

    for &c in visual {
        if c == '?' {
            return ArrangementType::Incomplete;
        } else if c == '.' {
            if group_size > 0 {
                if group_size != numeric[numeric_idx] {
                    return ArrangementType::Invalid;
                }
                numeric_idx += 1;
                group_size = 0;
            }
        } else if c == '#' {
            group_size += 1;
            if numeric_idx >= numeric.len() || group_size > numeric[numeric_idx] {
                return ArrangementType::Invalid;
            }
        }
    }

    if numeric_idx < numeric.len() {
        return ArrangementType::Invalid;
    }
    return ArrangementType::Valid;
}

fn count_arrangements(visual: &mut Vec<char>, numeric: &Vec<i32>) -> i32 {
    match consistent_arrangement(visual, numeric) {
        ArrangementType::Invalid => 0,
        ArrangementType::Valid => 1,
        ArrangementType::Incomplete => {
            let first_unknown = visual
                .iter()
                .enumerate()
                .find(|(_, c)| **c == '?')
                .unwrap()
                .0;

            let mut arrangements_sum = 0;
            visual[first_unknown] = '.';
            arrangements_sum += count_arrangements(visual, numeric);
            visual[first_unknown] = '#';
            arrangements_sum += count_arrangements(visual, numeric);
            visual[first_unknown] = '?';

            return arrangements_sum;
        }
    }
}

#[aoc(day12, part1)]
fn day12part1(input: &str) -> i32 {
    // let input = "???.### 1,1,3
    // .??..??...?##. 1,1,3
    // ?#?#?#?#?#?#?#? 1,3,1,6
    // ????.#...#... 4,1,1
    // ????.######..#####. 1,6,5
    // ?###???????? 3,2,1";

    let mut arrangement_sum = 0;

    // let mut tv: Vec<char> = vec!['.', '#', '#', '.', '#', '#', '#'];
    // let cc: Vec<i32> = vec![1, 1, 3];
    // consistent_arrangement(&mut tv, &cc);
    // return 5;

    for line in input.trim().lines() {
        let line = line.trim();
        let line: Vec<&str> = line.split(" ").collect();
        let mut visual: Vec<char> = line[0].chars().collect();
        // simply logic by always having termination
        visual.push('.');
        let numeric: Vec<i32> = line[1].split(",").map(|n| n.parse().unwrap()).collect();

        let arrangements = count_arrangements(&mut visual, &numeric);
        arrangement_sum += arrangements;
        println!("{visual:?} {numeric:?} {arrangements} {arrangement_sum}");
    }

    return arrangement_sum;
}
