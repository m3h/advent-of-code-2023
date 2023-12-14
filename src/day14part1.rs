use std::ops;

#[derive(Copy, Clone)]
struct Coordinate {
    y: i64,
    x: i64,
}

impl ops::Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, _rhs: Coordinate) -> Coordinate {
        Coordinate {
            y: self.y + _rhs.y,
            x: self.x + _rhs.x,
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn to_movement(&self) -> Coordinate {
        match self {
            Direction::North => Coordinate { y: -1, x: 0 },
            Direction::South => Coordinate { y: 1, x: 0 },
            Direction::East => Coordinate { y: 0, x: 1 },
            Direction::West => Coordinate { y: 0, x: -1 },
        }
    }
}

impl ops::Add<Direction> for Coordinate {
    type Output = Coordinate;

    fn add(self, _rhs: Direction) -> Coordinate {
        self + _rhs.to_movement()
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Rock {
    Round,
    Cube,
    Ground,
}

impl Rock {
    fn from_char(c: char) -> Rock {
        match c {
            '#' => Rock::Cube,
            'O' => Rock::Round,
            '.' => Rock::Ground,
            _ => panic!("Unknown Rock type: {c}"),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Rock::Cube => '#',
            Rock::Round => 'O',
            Rock::Ground => '.',
        }
    }
}

struct Platform {
    map: Vec<Vec<Rock>>,
}

impl Platform {
    fn from_str(input: &str) -> Platform {
        Platform {
            map: input
                .trim()
                .lines()
                .map(|line| line.trim().chars().map(|c| Rock::from_char(c)).collect())
                .collect(),
        }
    }

    fn is_valid(&self, p: &Coordinate) -> bool {
        p.x >= 0
            && p.y >= 0
            && p.y < self.map.len() as i64
            && p.x < self.map[p.y as usize].len() as i64
    }

    fn get(&self, p: &Coordinate) -> Rock {
        self.map[p.y as usize][p.x as usize]
    }

    fn set(&mut self, p: &Coordinate, v: Rock) {
        self.map[p.y as usize][p.x as usize] = v;
    }

    fn is_free(&self, dst: &Coordinate) -> bool {
        self.is_valid(dst) && self.get(dst) == Rock::Ground
    }

    fn try_roll(&mut self, src: &Coordinate, direction: Direction) {
        let dst = *src + direction;
        if self.get(src) == Rock::Round && self.is_free(&dst) {
            self.set(&dst, Rock::Round);
            self.set(src, Rock::Ground);
            self.try_roll(&dst, direction);
        }
    }

    fn roll(&mut self, direction: Direction) {
        match direction {
            Direction::North => {
                for y in 0..self.map.len() {
                    for x in 0..self.map[y].len() {
                        self.try_roll(
                            &Coordinate {
                                y: y as i64,
                                x: x as i64,
                            },
                            direction,
                        );
                    }
                }
            }
            Direction::South => {
                for y in (0..self.map.len()).rev() {
                    for x in 0..self.map[y].len() {
                        self.try_roll(
                            &Coordinate {
                                y: y as i64,
                                x: x as i64,
                            },
                            direction,
                        );
                    }
                }
            }

            Direction::East => {
                for x in (0..self.map[0].len()).rev() {
                    for y in 0..self.map.len() {
                        self.try_roll(
                            &Coordinate {
                                y: y as i64,
                                x: x as i64,
                            },
                            direction,
                        );
                    }
                }
            }

            Direction::West => {
                for x in 0..self.map[0].len() {
                    for y in 0..self.map.len() {
                        self.try_roll(
                            &Coordinate {
                                y: y as i64,
                                x: x as i64,
                            },
                            direction,
                        );
                    }
                }
            }
        }
    }

    fn total_load(&self) -> usize {
        let mut load = 0;
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                if self.map[y][x] == Rock::Round {
                    load += self.map.len() - y
                }
            }
        }
        return load;
    }

    fn to_str(&self) -> String {
        let mut s = String::new();

        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                s += &self.map[y][x].to_char().to_string();
            }
            s += "\n";
        }

        return s;
    }
}

#[aoc(day14, part1)]
fn day14part1(input: &str) -> usize {
    let mut platform = Platform::from_str(input);
    let platform_input_s = platform.to_str();
    println!("input:\n\n{platform_input_s}");

    platform.roll(Direction::North);

    let platform_rolled_north_s = platform.to_str();
    println!("rolled:\n\n{platform_rolled_north_s}");

    return platform.total_load();
}
