use std::collections::HashMap;

fn count_combinations(
    visual: &mut Vec<char>,
    numeric: &Vec<usize>,
    visual_idx: usize,
    numeric_idx: usize,
    group_size: usize,
    cache: &mut HashMap<(Vec<char>, Vec<usize>, usize), u64>,
) -> u64 {
    let cache_key = (
        visual[visual_idx..].to_vec(),
        numeric[numeric_idx..].to_vec(),
        group_size,
    );
    match cache.get(&cache_key) {
        Some(cache_hit) => return *cache_hit,
        None => {}
    };

    if visual_idx >= visual.len() {
        if (numeric_idx >= numeric.len() && group_size == 0)
            || (numeric_idx == numeric.len() - 1 && numeric[numeric_idx] == group_size)
        {
            cache.insert(cache_key, 1);
            return 1;
        }

        cache.insert(cache_key, 0);
        return 0;
    }

    if visual[visual_idx] == '?' {
        visual[visual_idx] = '#';
        let hash_count =
            count_combinations(visual, numeric, visual_idx, numeric_idx, group_size, cache);
        visual[visual_idx] = '.';
        let dot_count =
            count_combinations(visual, numeric, visual_idx, numeric_idx, group_size, cache);
        let sum = hash_count + dot_count;
        visual[visual_idx] = '?';

        cache.insert(cache_key, sum);
        return sum;
    }

    if visual[visual_idx] == '#' {
        if numeric_idx >= numeric.len() || group_size + 1 > numeric[numeric_idx] {
            cache.insert(cache_key, 0);
            return 0;
        }
        let c = count_combinations(
            visual,
            numeric,
            visual_idx + 1,
            numeric_idx,
            group_size + 1,
            cache,
        );
        cache.insert(cache_key, c);
        return c;
    } else if visual[visual_idx] == '.' {
        // terminate group
        let new_numeric_idx = if group_size != 0 {
            if group_size != numeric[numeric_idx] {
                cache.insert(cache_key, 0);
                return 0;
            }
            numeric_idx + 1
        } else {
            numeric_idx
        };
        let c = count_combinations(visual, numeric, visual_idx + 1, new_numeric_idx, 0, cache);
        cache.insert(cache_key, c);
        return c;
    }

    panic!("should be unreachable");
}

#[aoc(day12, part2)]
fn day12part2(input: &str) -> u64 {
    // let input = "???.### 1,1,3
    // .??..??...?##. 1,1,3
    // ?#?#?#?#?#?#?#? 1,3,1,6
    // ????.#...#... 4,1,1
    // ????.######..#####. 1,6,5
    // ?###???????? 3,2,1";

    let mut combinations = 0;
    for line in input.trim().lines() {
        let line = line.trim();

        let line_parts: Vec<&str> = line.split(" ").collect();
        let visual_blueprint: Vec<char> = line_parts[0].chars().collect();
        let numeric_blueprint: Vec<usize> = line_parts[1]
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect();

        let mut visual = Vec::new();
        let mut numeric = Vec::new();
        for _ in 0..5 {
            visual.extend(visual_blueprint.clone());
            numeric.extend(numeric_blueprint.clone());

            visual.push('?');
        }
        visual.pop();

        let line_combinations =
            count_combinations(&mut visual, &numeric, 0, 0, 0, &mut HashMap::new());
        combinations += line_combinations;
    }
    return combinations;
}
