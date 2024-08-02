use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn to_movement(&self) -> Coordinate {
        match self {
            Direction::UP => Coordinate { y: -1, x: 0 },
            Direction::DOWN => Coordinate { y: 1, x: 0 },
            Direction::RIGHT => Coordinate { y: 0, x: 1 },
            Direction::LEFT => Coordinate { y: 0, x: -1 },
        }
    }
}

fn map_at(map: &Vec<Vec<Tile>>, coordinate: Coordinate) -> Option<Tile> {
    if coordinate.x < 0
        || coordinate.y < 0
        || coordinate.y as usize >= map.len()
        || coordinate.x as usize >= map[coordinate.x as usize].len()
    {
        return None;
    }
    return Some(map[coordinate.y as usize][coordinate.x as usize]);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coordinate {
    y: i64,
    x: i64,
}

impl std::ops::Add<Direction> for Coordinate {
    type Output = Coordinate;

    fn add(self, _rhs: Direction) -> Coordinate {
        self + _rhs.to_movement()
    }
}

impl std::ops::Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, _rhs: Coordinate) -> Coordinate {
        Coordinate {
            y: self.y + _rhs.y,
            x: self.x + _rhs.x,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Tile {
    Path,
    Forest,
    SlopeRight,
    SlopeDown,
    SlopeLeft,
    SlopeUp,
}

impl Tile {
    fn allowed_directions(&self) -> Vec<Direction> {
        match self {
            Tile::Path => vec![
                Direction::UP,
                Direction::DOWN,
                Direction::LEFT,
                Direction::RIGHT,
            ],
            Tile::Forest => vec![],
            Tile::SlopeDown => vec![Direction::DOWN],
            Tile::SlopeUp => vec![Direction::UP],
            Tile::SlopeRight => vec![Direction::RIGHT],
            Tile::SlopeLeft => vec![Direction::LEFT],
        }
    }
}

fn get_neighbours(current: Coordinate, map: &Vec<Vec<Tile>>) -> Vec<Coordinate> {
    return map_at(map, current)
        .unwrap()
        .allowed_directions()
        .iter()
        .filter_map(|direction| {
            let new_coordinate = current + *direction;

            match map_at(map, new_coordinate) {
                Some(tile) => {
                    if tile == Tile::Forest {
                        return None;
                    }
                }
                None => return None,
            }

            return Some(new_coordinate);
        })
        .collect();
}

fn find_start(map: &Vec<Vec<Tile>>) -> Coordinate {
    let y = 0;
    for x in 0..map[y].len() {
        if map[y][x] == Tile::Path {
            return Coordinate {
                y: y as i64,
                x: x as i64,
            };
        }
    }
    panic!("no start!")
}
fn find_goal(map: &Vec<Vec<Tile>>) -> Coordinate {
    let y = map.len() - 1;
    for x in 0..map[y].len() {
        if map[y][x] == Tile::Path {
            return Coordinate {
                y: y as i64,
                x: x as i64,
            };
        }
    }
    panic!("no goal!")
}

fn seen(
    seen_cache: &mut std::collections::HashSet<u64>,
    depth: usize,
    coordinate: Coordinate,
    visited: &std::collections::HashMap<Coordinate, usize>,
) -> bool {
    let mut s = DefaultHasher::new();

    coordinate.hash(&mut s);
    depth.hash(&mut s);

    let mut visited = visited.iter().collect::<Vec<_>>();
    visited.sort_unstable();

    for (coordinate, visit_count) in visited.iter() {
        if **visit_count != 0 {
            coordinate.hash(&mut s);
            visit_count.hash(&mut s);
        }
    }

    let seen_key = s.finish();
    let has_been_seen = seen_cache.contains(&seen_key);
    seen_cache.insert(seen_key);

    return has_been_seen;
}
fn dfs(
    map: &Vec<Vec<Tile>>,
    start: Coordinate,
    goal: Coordinate,
    visited: &mut std::collections::HashMap<Coordinate, usize>,
    depth: usize,
    max_depth: usize,
    total_path_tiles: usize,
    seen_cache: &mut std::collections::HashSet<u64>,
) -> usize {
    if start == goal {
        if depth > max_depth {
            println!("depth: {depth}");
        }
        return depth;
    }

    // if visiting all remaining tiles is a shorter hike than what we've found, abort
    let upper_bound = 2 * total_path_tiles - depth;
    if max_depth >= upper_bound {
        println!("prune depth {max_depth} >= {upper_bound}");
        return usize::MIN;
    }
    if seen(seen_cache, depth, start, visited) {
        println!("prune seen");
        return usize::MIN;
    }

    if !visited.contains_key(&start) {
        visited.insert(start, 0);
    }
    let visited_count = *visited.get(&start).unwrap();
    visited.insert(start, visited_count + 1);
    // visited
    //     .entry(start)
    //     .and_modify(|count| *count += 1)
    //     .or_insert(1);

    let mut max_depth = max_depth;
    for potential_neighbour in get_neighbours(start, map) {
        if match visited.get(&potential_neighbour) {
            Some(count) => *count < 1,
            None => true,
        } {
            max_depth = max_depth.max(dfs(
                map,
                potential_neighbour,
                goal,
                visited,
                depth + 1,
                max_depth,
                total_path_tiles,
                seen_cache,
            ));
        }
    }

    visited.insert(start, visited_count);
    return max_depth;
}

struct Node {
    coordinate: Coordinate,
    depth: usize,
    visited: std::collections::HashMap<Coordinate, usize>,
}

fn bfs(
    map: &Vec<Vec<Tile>>,
    start: Coordinate,
    goal: Coordinate,
    total_path_tiles: usize,
) -> usize {
    let mut max_depth = usize::MIN;

    let mut queue = std::collections::VecDeque::from([Node {
        coordinate: start,
        depth: 1,
        visited: std::collections::HashMap::new(),
    }]);

    while !queue.is_empty() {
        let n = queue.pop_front().unwrap();

        if n.coordinate == goal {
            max_depth = max_depth.max(n.depth);
            continue;
        }

        let mut n_visited = n.visited.clone();
        n_visited
            .entry(n.coordinate)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        let potential_neighbours = get_neighbours(n.coordinate, map);
        for potential_neighbour in potential_neighbours {
            if match n_visited.get(&potential_neighbour) {
                Some(count) => *count <= 1,
                None => true,
            } {
                queue.push_back(Node {
                    coordinate: potential_neighbour,
                    depth: n.depth + 1,
                    visited: n_visited.clone(),
                })
            }
        }
    }

    return max_depth;
}

#[aoc(day23, part1)]
fn day23part2(input: &str) -> usize {
    let input = "#.#####################
    #.......#########...###
    #######.#########.#.###
    ###.....#.>.>.###.#.###
    ###v#####.#v#.###.#.###
    ###.>...#.#.#.....#...#
    ###v###.#.#.#########.#
    ###...#.#.#.......#...#
    #####.#.#.#######.#.###
    #.....#.#.#.......#...#
    #.#####.#.#.#########v#
    #.#...#...#...###...>.#
    #.#.#v#######v###.###v#
    #...#.>.#...>.>.#.###.#
    #####v#.#.###v#.#.###.#
    #.....#...#...#.#.#...#
    #.#########.###.#.#.###
    #...###...#...#...#.###
    ###.###.#.###v#####v###
    #...#...#.#.>.>.#.>.###
    #.###.###.#.###.#.#v###
    #.....###...###...#...#
    #####################.#";

    let map: Vec<Vec<Tile>> = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '.' => Tile::Path,
                    '#' => Tile::Forest,
                    '^' => Tile::Path,
                    '>' => Tile::Path,
                    'v' => Tile::Path,
                    '<' => Tile::Path,
                    _ => panic!("Unknown tile type"),
                })
                .collect()
        })
        .collect();

    let mut total_path_tiles = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == Tile::Path {
                total_path_tiles += 1;
            }
        }
    }

    // return bfs(&map, find_start(&map), find_goal(&map), total_path_tiles);
    return dfs(
        &map,
        find_start(&map),
        find_goal(&map),
        &mut std::collections::HashMap::new(),
        0,
        usize::MIN,
        total_path_tiles,
        &mut std::collections::HashSet::new(),
    );
}
