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
}

impl Direction {
    fn to_movement(&self) -> Coordinate {
        match self {
            Direction::North => Coordinate { y: -1, x: 0 },
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
}

#[aoc(day14, part1)]
fn day14part1(input: &str) -> usize {
    let mut platform = Platform::from_str(input);

    platform.roll(Direction::North);

    return platform.total_load();
}
