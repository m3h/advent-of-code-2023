use std::fs;

use std::collections::HashMap;

fn is_prime(n: u128, prime_cache: &mut Vec<u128>) -> bool {
    for i in 0.. {
        let pi = get_prime(i, prime_cache);
        if n % pi == 0 {
            return false;
        }
        if pi >= n / 2 {
            return true;
        }
    }
    // unreachable
    return true;
}

fn get_prime(idx: usize, prime_cache: &mut Vec<u128>) -> u128 {
    if prime_cache.len() == 0 {
        prime_cache.push(2);
    }

    while idx >= prime_cache.len() {
        let last_prime = *prime_cache.last().unwrap();

        let mut next_candidate= last_prime+1;
        while !is_prime(next_candidate, prime_cache) {
            next_candidate += 1;
        }
        prime_cache.push(next_candidate);
    }

    return prime_cache[idx];
}

fn prime_factorize(input_num: u128, prime_cache: &mut Vec<u128>) -> HashMap<u128, u128> {
    let mut n = input_num;
    let mut prime_idx = 0;
    let mut prime = get_prime(prime_idx, prime_cache);

    let mut prime_factors = HashMap::new();
    while n != 1 {
        if n % prime == 0 {
            if !prime_factors.contains_key(&prime) {
                prime_factors.insert(prime, 0);
            }
            prime_factors.insert(prime, prime_factors.get(&prime).unwrap() + 1);
            n /= prime;
            prime_idx = 0;
            prime = get_prime(prime_idx, prime_cache);
        } else {
            prime_idx += 1;
            prime = get_prime(prime_idx, prime_cache);
        }
    }
    return prime_factors;
}

fn lcm(nums: Vec<u128>, prime_cache: &mut Vec<u128>) -> u128 {

    let mut common_factors: HashMap<u128, u128> = HashMap::new();

    for num in nums {
        let num_prime_factors = prime_factorize(num, prime_cache);

        for pf in num_prime_factors.keys() {
            if !common_factors.contains_key(pf) {
                common_factors.insert(*pf, 0);
            }

            let pf_count = num_prime_factors[pf];
            let previous_max = *common_factors.get(pf).unwrap();

            let new_max = pf_count.max(previous_max);

            common_factors.insert(*pf, new_max);
        }
    }

    let mut p = 1;
    for pf in common_factors.keys() {
        p *= *pf * common_factors.get(pf).unwrap();
    }
    return p;

}
fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    // let input = "LR

    // 11A = (11B, XXX)
    // 11B = (XXX, 11Z)
    // 11Z = (11B, XXX)
    // 22A = (22B, XXX)
    // 22B = (22C, 22C)
    // 22C = (22Z, 22Z)
    // 22Z = (22B, 22B)
    // XXX = (XXX, XXX)";

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

    let mut locs: Vec<_> = map.keys().filter(|k| k.ends_with("A")).map(|k| k.clone()).collect();

    let mut step_counts: Vec<Vec<u128>> = Vec::new();
    for _ in 0..locs.len() {
        step_counts.push(Vec::new());
    }

    let mut step_count = 0;
    let mut instruction_idx = 0;

    while locs.iter().any(|k| k != "") {
        println!("{locs:?}");
        step_count += 1;
        let instruction = instructions[instruction_idx];
        instruction_idx = (instruction_idx + 1) % instructions.len();

        for i in 0..locs.len() {
            if locs[i] == "" {
                continue;
            }
            if instruction == b'L' {
                locs[i] = map.get(&locs[i]).unwrap().0.clone();
            } else if instruction == b'R' {
                locs[i] = map.get(&locs[i]).unwrap().1.clone();
            } else {
                panic!("oh no! {instruction}");
            }

            if locs[i].ends_with("Z") {
                // don't grab duplicates
                for previous_step_count in step_counts[i].iter() {
                    if step_count % previous_step_count == 0 {
                        locs[i] = "".to_string();
                    }
                }
                if locs[i] != "" {
                    step_counts[i].push(step_count);
                }
            }
        }
    }

    // at this point I cheated and used a website:
    // https://www.calculatorsoup.com/calculators/math/lcm.php
    println!("{step_counts:?}");

    // but let me continue
    let mut prime_cache = Vec::new();
    let step_counts = step_counts.iter().map(|s| s[0]).collect();
    let ans = lcm(step_counts, &mut prime_cache);

    println!("{ans}");

}
