use std::fmt;

#[derive(PartialEq, Debug, Clone, Copy)]
enum PipeType {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    GROUND,
    START,
}

#[derive(Debug)]
struct Movement {
    y: i32,
    x: i32,
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Point {
    y: i32,
    x: i32,
}

#[derive(Debug)]
struct PipeMap {
    pipes: Vec<Vec<PipeType>>,
    start_point: Point,
}

impl PipeMap {
    fn from_str_map(str_map: &str) -> PipeMap {
        let str_map = str_map.trim();
        let str_map: Vec<&str> = str_map.lines().collect();
        let str_map: Vec<Vec<char>> = str_map
            .iter()
            .map(|line| line.chars().filter(|c| !c.is_whitespace()).collect())
            .collect();
        let pipes: Vec<Vec<PipeType>> = str_map
            .iter()
            .map(|line| line.iter().map(|c| PipeType::from_char(*c)).collect())
            .collect();

        let start_point = PipeMap::find_first_pipe_type(PipeType::START, &pipes).unwrap();

        return PipeMap { pipes, start_point };
    }

    fn find_first_pipe_type(pipe_type: PipeType, pipes: &Vec<Vec<PipeType>>) -> Option<Point> {
        for y in 0..pipes.len() {
            for x in 0..pipes[y].len() {
                if pipes[y][x] == pipe_type {
                    return Some(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }
        return None;
    }

    fn possible_neighbours_absolute(&self, point: &Point) -> Vec<Point> {
        let cur_pipe_type = self.type_at(point);

        let possible_neighbours = cur_pipe_type
            .possible_neighbours_relative()
            .iter()
            .filter_map(|movement| point.apply_movement(movement, &self.pipes))
            .collect();
        return possible_neighbours;
    }

    fn neighbours(&self, point: &Point) -> Vec<Point> {
        let possible_neighbours = self.possible_neighbours_absolute(point);

        possible_neighbours
            .iter()
            .filter(|neighbour| self.possible_neighbours_absolute(neighbour).contains(point))
            .map(|neighbour| *neighbour)
            .collect()
    }

    fn type_at(&self, point: &Point) -> PipeType {
        return self.pipes[point.y as usize][point.x as usize];
    }

    fn loop_length(&self) -> i32 {
        let mut steps = 0;
        let mut previous_point = self.start_point;
        let mut cur_point = self.start_point;
        while steps == 0 || self.type_at(&cur_point) != PipeType::START {
            let next_point = *self
                .neighbours(&cur_point)
                .iter()
                .filter(|&&p| p != previous_point)
                .next()
                .unwrap();

            steps += 1;
            previous_point = cur_point;
            cur_point = next_point;
        }

        return steps;
    }
}

impl Point {
    fn apply_movement(&self, movement: &Movement, map: &Vec<Vec<PipeType>>) -> Option<Point> {
        let new_loc = Point {
            x: self.x + movement.x,
            y: self.y + movement.y,
        };

        if new_loc.x < 0
            || new_loc.y < 0
            || new_loc.y as usize >= map.len()
            || map.len() < 1
            || new_loc.x as usize >= map[0].len()
        {
            return None;
        } else {
            return Some(new_loc);
        }
    }
}

impl fmt::Display for PipeType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            PipeType::NS => "|",
            PipeType::EW => "-",
            PipeType::NE => "L",
            PipeType::NW => "J",
            PipeType::SW => "7",
            PipeType::SE => "F",
            PipeType::GROUND => ".",
            PipeType::START => "S",
        })
    }
}

impl PipeType {
    fn from_char(c: char) -> PipeType {
        match c {
            '|' => PipeType::NS,
            '-' => PipeType::EW,
            'L' => PipeType::NE,
            'J' => PipeType::NW,
            '7' => PipeType::SW,
            'F' => PipeType::SE,
            '.' => PipeType::GROUND,
            'S' => PipeType::START,
            _ => panic!("Unknown pipe type!"),
        }
    }

    fn possible_neighbours_relative(&self) -> Vec<Movement> {
        match *self {
            PipeType::NS => Vec::from([Movement { x: 0, y: 1 }, Movement { x: 0, y: -1 }]),
            PipeType::EW => Vec::from([Movement { x: -1, y: 0 }, Movement { x: 1, y: 0 }]),
            PipeType::NE => Vec::from([Movement { x: 0, y: -1 }, Movement { x: 1, y: 0 }]),
            PipeType::NW => Vec::from([Movement { x: 0, y: -1 }, Movement { x: -1, y: 0 }]),
            PipeType::SW => Vec::from([Movement { x: 0, y: 1 }, Movement { x: -1, y: 0 }]),
            PipeType::SE => Vec::from([Movement { x: 0, y: 1 }, Movement { x: 1, y: 0 }]),
            PipeType::GROUND => Vec::new(),
            PipeType::START => Vec::from([
                Movement { x: 0, y: 1 },
                Movement { x: 1, y: 0 },
                Movement { x: 0, y: -1 },
                Movement { x: -1, y: 0 },
            ]),
        }
    }
}

#[aoc(day10, part1)]
fn day10part1(input: &str) -> i32 {
    // let input = ".....
    // .S-7.
    // .|.|.
    // .L-J.
    // .....";

    let pipe_map = PipeMap::from_str_map(&input);
    let loop_steps = pipe_map.loop_length();
    let ans = loop_steps / 2;
    return ans;
}
