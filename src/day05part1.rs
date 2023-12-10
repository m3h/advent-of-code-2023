#[aoc(day5, part1)]
fn day05part1(input: &str) -> u128 {
    let mut seeds = Vec::new();
    let mut current_src: String = "".to_string();
    let mut current_dst: String = "".to_string();
    let mut maps = Vec::new();
    // let mut maps: HashMap<(String, String), (u128, u128)> = HashMap::new();
    for line in input.lines() {
        let line = line.trim();
        if line == "" {
            continue;
        }

        if line.contains("seeds:") {
            seeds = line
                .split(": ")
                .last()
                .unwrap()
                .split(" ")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u128>>();
        } else if line.contains(":") {
            let types = line.split(" ").collect::<Vec<&str>>()[0];
            let src_type = types.split("-").collect::<Vec<&str>>()[0];
            let dst_type = types.split("-").collect::<Vec<&str>>()[2];

            current_src = src_type.to_string();
            current_dst = dst_type.to_string();
        } else {
            let loc_specifier = line
                .split(" ")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u128>>();
            let start_init = loc_specifier[1];
            let dst_init = loc_specifier[0];
            let range_size = loc_specifier[2];

            maps.push((
                current_src.clone(),
                current_dst.clone(),
                start_init,
                dst_init,
                range_size,
            ))
        }
    }

    let mut current_loc_type = "seed".to_string();
    let mut locs = seeds.clone();

    while current_loc_type != "location" {
        // by default, same indices
        let mut new_locs = locs.clone();
        let mut new_loc_type = "".to_string();
        for (src, dst, s, d, range_size) in &maps {
            if *src == current_loc_type {
                new_loc_type = dst.clone();
                for i in 0..locs.len() {
                    if *s <= locs[i] && locs[i] <= *s + *range_size {
                        new_locs[i] = *d + locs[i] - *s;
                    }
                }
            }
        }

        current_loc_type = new_loc_type;
        locs = new_locs;
    }

    let min_loc = locs.iter().min().unwrap();
    return *min_loc;
}
