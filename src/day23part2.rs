use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    UP = 0,
    DOWN = 1,
    LEFT = 2,
    RIGHT = 3,
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Tile {
    Forest,
    Path,
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

#[derive(PartialEq, Eq, Clone, PartialOrd, Ord)]
struct Node {
    coordinate: Coordinate,
    visited: Vec<Coordinate>,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coordinate.hash(state);
        self.visited.hash(state);
    }
}

impl Node {
    fn new(c: Coordinate, visited: &Vec<Coordinate>) -> Node {
        let mut visited = visited.clone();
        visited.sort_unstable();

        return Node {
            coordinate: c,
            visited: visited,
        };
    }

    fn visit(&self, c: Coordinate) -> Node {
        let mut n = self.clone();
        n.visited.push(c);
        n.visited.sort_unstable();
        return n;
    }
    fn neighbours(&self, map: &Vec<Vec<Tile>>) -> Vec<Node> {
        [
            Direction::UP,
            Direction::DOWN,
            Direction::LEFT,
            Direction::RIGHT,
        ]
        .iter()
        .filter_map(|direction| {
            let c = self.coordinate + *direction;

            if c.y < 0 || c.x < 0 || c.y as usize >= map.len() || c.x as usize >= map[0].len() {
                return None;
            }
            if map[c.y as usize][c.x as usize] == Tile::Forest {
                return None;
            }
            if self.visited.contains(&c) {
                return None;
            }

            return Some(Node::new(c, &self.visited).visit(c));
        })
        .collect()
    }
}

fn get_vertices(
    start: &Node,
    map: &Vec<Vec<Tile>>,
    vertices: &mut std::collections::HashSet<Node>,
) {
    vertices.insert(start.clone());

    for neighbour in start.neighbours(map) {
        get_vertices(&neighbour, map, vertices);
    }
}

struct Graph {
    start: Node,
    goal: Coordinate,
    vertices: std::collections::HashSet<Node>,
    map: Vec<Vec<Tile>>,
}

impl Graph {
    fn from_str(input: &str) -> Graph {
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

        let start = find_start(&map);
        let goal = find_goal(&map);

        let start = Node::new(start, &vec![start]);
        let mut vertices = std::collections::HashSet::new();
        get_vertices(&start, &map, &mut vertices);

        return Graph {
            start,
            goal,
            vertices,
            map,
        };
    }

    fn dijkstra(&self) -> i64 {
        let mut dist = std::collections::HashMap::new();

        // let mut q = std::collections::BinaryHeap::new();
        let mut q = Vec::new();

        for v in &self.vertices {
            dist.insert(v.clone(), i64::MIN);
            q.push(v.clone());
        }
        *dist.get_mut(&self.start).unwrap() = 0;

        while q.len() > 0 {
            q.sort_unstable_by(|a, b| dist.get(a).unwrap().cmp(dist.get(b).unwrap()));
            let u = q.pop().unwrap();

            for v in u.neighbours(&self.map) {
                let alt = dist.get(&u).unwrap() + 1;
                if alt > *dist.get(&v).unwrap() {
                    dist.insert(v, alt);
                }
            }
        }

        let mut goal_nodes = dist
            .iter()
            .filter(|(n, _)| n.coordinate == self.goal)
            .collect::<Vec<_>>();
        goal_nodes.sort_by(|a, b| b.1.cmp(a.1));

        return *goal_nodes.get(0).unwrap().1;
    }
}
#[aoc(day23, part2)]
fn day23part2(input: &str) -> i64 {
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

    // return bfs(&map, find_start(&map), find_goal(&map), total_path_tiles);

    let graph = Graph::from_str(input);

    return graph.dijkstra();
}
