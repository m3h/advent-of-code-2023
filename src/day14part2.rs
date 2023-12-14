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
}

struct Platform {
    past: Vec<Vec<Vec<Rock>>>,
    map: Vec<Vec<Rock>>,
}

impl Platform {
    fn from_str(input: &str) -> Platform {
        let blueprint: Vec<Vec<Rock>> = input
            .trim()
            .lines()
            .map(|line| line.trim().chars().map(|c| Rock::from_char(c)).collect())
            .collect();
        return Platform {
            past: vec![blueprint.clone()],
            map: blueprint.clone(),
        };
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

    fn spin_cycle(&mut self) {
        self.roll(Direction::North);
        self.roll(Direction::West);
        self.roll(Direction::South);
        self.roll(Direction::East);

        self.past.push(self.map.clone());
    }

    fn equals_past(&self) -> Option<usize> {
        'past_loop: for i in 0..(self.past.len() - 1) {
            for y in 0..self.map.len() {
                for x in 0..self.map[y].len() {
                    if self.map[y][x] != self.past[i][y][x] {
                        continue 'past_loop;
                    }
                }
            }
            return Some(i);
        }
        return None;
    }
}

#[aoc(day14, part2)]
fn day14part2(input: &str) -> usize {
    // let input = "O....#....
    // O.OO#....#
    // .....##...
    // OO.#O....O
    // .O.....O#.
    // O.#..O.#.#
    // ..O..#O..O
    // .......O..
    // #....###..
    // #OO..#....";

    let mut platform = Platform::from_str(input);

    let mut cycle = 0;
    let total_cycles = 1_000_000_000;
    while cycle < total_cycles {
        platform.spin_cycle();

        match platform.equals_past() {
            Some(i) => {
                let loop_size = cycle - i + 1;
                let remaining_cycles = total_cycles - cycle;
                let remaining_loop_iterations = remaining_cycles / loop_size;
                cycle += loop_size * remaining_loop_iterations;
            }
            None => {}
        }
        cycle += 1;
    }

    return platform.total_load();
}
