#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn from_str(s: &str) -> Direction {
        match s {
            "D" => Direction::DOWN,
            "U" => Direction::UP,
            "L" => Direction::LEFT,
            "R" => Direction::RIGHT,
            _ => panic!("unknown direction!"),
        }
    }
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum TileType {
    GROUND,
    HOLE,
    AIR,
}
struct LavaMap {
    map: Vec<Vec<TileType>>,
    position: Coordinate,
}

impl LavaMap {
    fn new() -> LavaMap {
        return LavaMap {
            map: vec![vec![TileType::GROUND]],
            position: Coordinate { y: 0, x: 0 },
        };
    }
    fn from_str(input: &str) -> usize {
        let mut lava_map = LavaMap::new();

        for (direction, steps) in input.trim().lines().map(|line| {
            let line_parts: Vec<&str> = line.split_whitespace().collect();

            let direction = Direction::from_str(line_parts[0]);
            let steps: usize = line_parts[1].parse().unwrap();

            return (direction, steps);
        }) {
            lava_map.insert(direction, steps, TileType::HOLE);
        }

        lava_map.fill();

        return lava_map.count();
    }

    fn insert_single(&mut self, direction: Direction, tile: TileType) {
        self.position = self.position + direction;
        while self.position.y < 0 {
            self.map.insert(0, Vec::new());
            self.position.y += 1;
        }
        while self.position.x < 0 {
            for y in 0..self.map.len() {
                self.map[y].insert(0, TileType::GROUND);
            }
            self.position.x += 1;
        }
        while self.position.y as usize >= self.map.len() {
            self.map.push(Vec::new());
        }
        while self.position.x as usize >= self.map[self.position.y as usize].len() {
            self.map[self.position.y as usize].push(TileType::GROUND);
        }

        self.map[self.position.y as usize][self.position.x as usize] = tile;
    }
    fn insert(&mut self, direction: Direction, steps: usize, tile: TileType) {
        for _ in 0..steps {
            self.insert_single(direction, tile);
        }
    }

    fn max_rows(&self) -> usize {
        return self.map.len();
    }

    fn max_cols(&self) -> usize {
        let mut cols = usize::MIN;
        for row in &self.map {
            cols = cols.max(row.len());
        }
        return cols;
    }

    fn fill(&mut self) {
        // add border of GROUND
        self.position = Coordinate { y: 0, x: 0 };
        self.insert(Direction::UP, 1, TileType::GROUND);
        self.insert(Direction::RIGHT, self.max_cols(), TileType::GROUND);
        self.insert(Direction::DOWN, self.max_rows(), TileType::GROUND);
        self.insert(Direction::LEFT, self.max_cols(), TileType::GROUND);
        self.insert(Direction::UP, self.max_rows(), TileType::GROUND);

        // replace outside GROUND with AIR
        // we know 0,0 is outside, because we just filled it with GROUND
        self.airify(Coordinate { y: 0, x: 0 });

        // any remaining GROUND is inside
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                if self.map[y][x] == TileType::GROUND {
                    self.map[y][x] = TileType::HOLE;
                }
            }
        }
    }

    fn airify(&mut self, start: Coordinate) {
        if self.map[start.y as usize][start.x as usize] == TileType::GROUND {
            self.map[start.y as usize][start.x as usize] = TileType::AIR;

            for direction in &[
                Direction::UP,
                Direction::RIGHT,
                Direction::DOWN,
                Direction::LEFT,
            ] {
                let neighbour = start + *direction;
                if neighbour.y >= 0
                    && neighbour.x >= 0
                    && (neighbour.y as usize) < self.map.len()
                    && (neighbour.x as usize) < self.map[neighbour.y as usize].len()
                {
                    self.airify(neighbour);
                }
            }
        }
    }

    fn count(&self) -> usize {
        let mut sum = 0;
        for row in &self.map {
            for tile in row {
                if *tile == TileType::HOLE {
                    sum += 1;
                }
            }
        }
        return sum;
    }
}
#[aoc(day18, part1)]
fn day18part1(input: &str) -> usize {
    // let input = "R 6 (#70c710)
    // D 5 (#0dc571)
    // L 2 (#5713f0)
    // D 2 (#d2c081)
    // R 2 (#59c680)
    // D 2 (#411b91)
    // L 5 (#8ceee2)
    // U 2 (#caa173)
    // L 1 (#1b58a2)
    // U 2 (#caa171)
    // R 2 (#7807d2)
    // U 3 (#a77fa3)
    // L 2 (#015232)
    // U 2 (#7a21e3)";

    return LavaMap::from_str(input);
}
