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

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    // N.B.! order is priority
    cost: usize,
    direction_count: usize,
    direction: Direction,
    coordinate: Coordinate,
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

impl State {
    fn marker(&self) -> (usize, Direction, Coordinate) {
        return (self.direction_count, self.direction, self.coordinate);
    }
}

fn get_neighbours(current: State, map: &Vec<Vec<usize>>) -> Vec<State> {
    let backwards = match current.direction {
        Direction::UP => Direction::DOWN,
        Direction::RIGHT => Direction::LEFT,
        Direction::DOWN => Direction::UP,
        Direction::LEFT => Direction::RIGHT,
    };

    return [
        Direction::UP,
        Direction::DOWN,
        Direction::LEFT,
        Direction::RIGHT,
    ]
    .iter()
    .filter_map(|new_direction| {
        let new_coordinate = current.coordinate + *new_direction;

        let new_direction_count = if *new_direction == current.direction {
            current.direction_count + 1
        } else {
            1
        };
        if new_coordinate.y < 0
            || new_coordinate.x < 0
            || new_coordinate.y as usize >= map.len()
            || new_coordinate.x as usize >= map[0].len()
            || new_direction_count > 3
            || *new_direction == backwards
        {
            return None;
        }

        let potential_neighbour = State {
            cost: current.cost + map[new_coordinate.y as usize][new_coordinate.x as usize],
            direction_count: new_direction_count,
            direction: *new_direction,
            coordinate: new_coordinate,
        };
        return Some(potential_neighbour);
    })
    .collect();
}
fn ucs(map: &Vec<Vec<usize>>, start: Coordinate, goal: Coordinate) -> usize {
    let start_node = State {
        cost: 0,
        direction_count: 1,
        direction: Direction::RIGHT,
        coordinate: start,
    };
    let mut start_node_down = start_node;
    start_node_down.direction = Direction::DOWN;

    let mut frontier = std::collections::BinaryHeap::from([start_node, start_node_down]);

    let mut expanded = std::collections::HashSet::new();

    while frontier.len() > 0 {
        let node = frontier.pop().unwrap();
        if node.coordinate == goal {
            return node.cost;
        }
        if expanded.contains(&node.marker()) {
            continue;
        }
        expanded.insert(node.marker());

        for neighbour in get_neighbours(node, map) {
            frontier.push(neighbour);
        }
    }

    panic!("Failed to find goal!");
}

#[aoc(day17, part1)]
fn day17part1(input: &str) -> usize {
    let map: Vec<Vec<usize>> = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect();

    return ucs(
        &map,
        Coordinate { y: 0, x: 0 },
        Coordinate {
            y: (map.len() - 1) as i64,
            x: (map[0].len() - 1) as i64,
        },
    );
}
