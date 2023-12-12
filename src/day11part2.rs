const GALAXY: char = '#';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

fn expand(graph: &Vec<Vec<char>>) -> (Vec<bool>, Vec<bool>) {
    let mut row_expanded: Vec<bool> = vec![true; graph.len()];
    let mut col_expanded: Vec<bool> = vec![true; graph[0].len()];

    for y in 0..graph.len() {
        if graph[y].iter().any(|&c| c == GALAXY) {
            row_expanded[y] = false;
        }
    }
    for x in 0..graph[0].len() {
        for y in 0..graph.len() {
            if graph[y][x] == GALAXY {
                col_expanded[x] = false;
            }
        }
    }

    return (row_expanded, col_expanded);
}

fn find_galaxies(graph: &Vec<Vec<char>>) -> Vec<Point> {
    let mut galaxies = vec![];
    for y in 0..graph.len() {
        for x in 0..graph.len() {
            if graph[y][x] == GALAXY {
                galaxies.push(Point { y, x });
            }
        }
    }
    return galaxies;
}

fn shortest_path(
    g1: &Point,
    g2: &Point,
    row_expansions: &Vec<bool>,
    col_expansions: &Vec<bool>,
) -> usize {
    let mut steps = 0;

    let mut p = *g1;

    while p != *g2 {
        if p.y != g2.y {
            let mult = if row_expansions[p.y] { 1000000 } else { 1 };
            p.y = (p.y as i64 + if p.y > g2.y { -1 } else { 1 }) as usize;
            steps += mult as usize;
        }
        if p.x != g2.x {
            let mult = if col_expansions[p.x] { 1000000 } else { 1 };
            p.x = (p.x as i64 + if p.x > g2.x { -1 } else { 1 }) as usize;
            steps += mult as usize;
        }
    }
    return steps;
}
#[aoc(day11, part2)]
fn day11part2(input: &str) -> usize {
    // let input = "...#......
    // .......#..
    // #.........
    // ..........
    // ......#...
    // .#........
    // .........#
    // ..........
    // .......#..
    // #...#.....";

    let star_map: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let (row_expansions, col_expansions) = expand(&star_map);
    let galaxies = find_galaxies(&star_map);

    let mut path_sum = 0;
    for g1 in galaxies.iter() {
        for g2 in galaxies.iter() {
            let path_len = shortest_path(g1, g2, &row_expansions, &col_expansions);
            path_sum += path_len;
        }
    }
    // oops double counted
    return path_sum / 2;
}
