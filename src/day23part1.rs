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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    // N.B.! order is priority
    cost: usize,
    coordinate: Coordinate,
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

fn dfs(
    map: &Vec<Vec<Tile>>,
    start: Coordinate,
    goal: Coordinate,
    visited: &mut std::collections::HashSet<Coordinate>,
) -> usize {
    if start == goal {
        return visited.len();
    }

    visited.insert(start);

    let mut max_depth = usize::MIN;
    for potential_neighbour in get_neighbours(start, map) {
        if !visited.contains(&potential_neighbour) {
            max_depth = max_depth.max(dfs(map, potential_neighbour, goal, visited));
        }
    }

    visited.remove(&start);
    return max_depth;
}
#[aoc(day23, part1)]
fn day23part1(input: &str) -> usize {
    // let input = "#.#####################
    // #.......#########...###
    // #######.#########.#.###
    // ###.....#.>.>.###.#.###
    // ###v#####.#v#.###.#.###
    // ###.>...#.#.#.....#...#
    // ###v###.#.#.#########.#
    // ###...#.#.#.......#...#
    // #####.#.#.#######.#.###
    // #.....#.#.#.......#...#
    // #.#####.#.#.#########v#
    // #.#...#...#...###...>.#
    // #.#.#v#######v###.###v#
    // #...#.>.#...>.>.#.###.#
    // #####v#.#.###v#.#.###.#
    // #.....#...#...#.#.#...#
    // #.#########.###.#.#.###
    // #...###...#...#...#.###
    // ###.###.#.###v#####v###
    // #...#...#.#.>.>.#.>.###
    // #.###.###.#.###.#.#v###
    // #.....###...###...#...#
    // #####################.#";

    let map: Vec<Vec<Tile>> = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '.' => Tile::Path,
                    '#' => Tile::Forest,
                    '^' => Tile::SlopeUp,
                    '>' => Tile::SlopeRight,
                    'v' => Tile::SlopeDown,
                    '<' => Tile::SlopeLeft,
                    _ => panic!("Unknown tile type"),
                })
                .collect()
        })
        .collect();

    return dfs(
        &map,
        find_start(&map),
        find_goal(&map),
        &mut std::collections::HashSet::new(),
    );
}
