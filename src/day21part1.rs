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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    // N.B.! order is priority
    cost: usize,
    coordinate: Coordinate,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Tile {
    GROUND,
    ROCK,
    START,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.cost.cmp(&other.cost) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
            std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_neighbours(current: State, map: &Vec<Vec<Tile>>, max_steps: usize) -> Vec<State> {
    return [
        Direction::UP,
        Direction::DOWN,
        Direction::LEFT,
        Direction::RIGHT,
    ]
    .iter()
    .filter_map(|direction| {
        let new_coordinate = current.coordinate + *direction;

        if new_coordinate.y < 0
            || new_coordinate.x < 0
            || new_coordinate.y as usize >= map.len()
            || new_coordinate.x as usize >= map[0].len()
            || map[new_coordinate.y as usize][new_coordinate.x as usize] == Tile::ROCK
            || current.cost > max_steps
        {
            return None;
        }

        let potential_neighbour = State {
            cost: current.cost + 1,
            coordinate: new_coordinate,
        };
        return Some(potential_neighbour);
    })
    .collect();
}

fn find_start(map: &Vec<Vec<Tile>>) -> Coordinate {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == Tile::START {
                return Coordinate {
                    y: y as i64,
                    x: x as i64,
                };
            }
        }
    }
    panic!("no start!")
}

fn visualize_visited(map: &Vec<Vec<Tile>>, visited: &std::collections::HashSet<Coordinate>) {
    let mut map = map.clone();
    for n in visited {
        map[n.y as usize][n.x as usize] = Tile::START;
    }
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let c = match map[y][x] {
                Tile::GROUND => ".",
                Tile::ROCK => "#",
                Tile::START => "O",
            };
            print!("{c}");
        }
        println!();
    }
}
fn ucs(map: &Vec<Vec<Tile>>, max_steps: usize) -> usize {
    let start = find_start(map);
    let start_node = State {
        cost: 0,
        coordinate: start,
    };

    let mut frontier = std::collections::BinaryHeap::from([start_node]);

    let mut expanded = std::collections::HashSet::new();

    let mut visited_coordinates = std::collections::HashSet::from([start]);

    while frontier.len() > 0 {
        let node = frontier.pop().unwrap();
        if expanded.contains(&node) {
            continue;
        }
        expanded.insert(node.clone());

        println!("{0}", node.cost);
        if node.cost == max_steps {
            visited_coordinates.insert(node.coordinate);
        }

        let neighbours = get_neighbours(node, map, max_steps);
        for neighbour in neighbours {
            frontier.push(neighbour);
        }
    }

    visualize_visited(map, &visited_coordinates);
    return visited_coordinates.len();
}

#[aoc(day21, part1)]
fn day21part1(input: &str) -> usize {
    // let input = "...........
    // .....###.#.
    // .###.##..#.
    // ..#.#...#..
    // ....#.#....
    // .##..S####.
    // .##..#...#.
    // .......##..
    // .##.#.####.
    // .##..##.##.
    // ...........";

    let map: Vec<Vec<Tile>> = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '.' => Tile::GROUND,
                    'S' => Tile::START,
                    '#' => Tile::ROCK,
                    _ => panic!("Unknown tile type"),
                })
                .collect()
        })
        .collect();

    return ucs(&map, 64);
}
