use std::fmt;

use aoc_runner_derive::aoc;

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

    fn mark_loop(&mut self) {
        let mut loop_marker: Vec<Vec<bool>> =
            vec![vec![false; self.pipes[0].len()]; self.pipes.len()];

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
            loop_marker[cur_point.y as usize][cur_point.x as usize] = true;
            steps += 1;
            previous_point = cur_point;
            cur_point = next_point;

            // self.print_map(cur_point);
        }

        // remove non-loopy pipes
        for y in 0..self.pipes.len() {
            for x in 0..self.pipes[y].len() {
                if !loop_marker[y][x] {
                    self.pipes[y][x] = PipeType::GROUND;
                }
            }
        }

        // replace START marker with appropriate pipe
        let real_pipe_types = [
            PipeType::NS,
            PipeType::EW,
            PipeType::NE,
            PipeType::NW,
            PipeType::SW,
            PipeType::SE,
        ];

        let mut start_type_found = false;
        for potential_start_type in real_pipe_types {
            self.pipes[self.start_point.y as usize][self.start_point.x as usize] =
                potential_start_type;

            if self.neighbours(&self.start_point).len() == 2 {
                start_type_found = true;
                break;
            }
        }
        if !start_type_found {
            panic!("error in start type logic");
        }
    }

    fn get_start_and_end(&self, init: Point) -> (bool, i32) {
        let mut visited = vec![];

        let mut q = vec![init];

        let mut ret = vec![];

        while q.len() > 0 {
            let p = q.pop().unwrap();
            if visited.contains(&p) {
                continue;
            } else {
                visited.push(p);
            }
            let neighbours = self.neighbours(&p);

            for neighbour in neighbours {
                if neighbour.y != init.y {
                    ret.push(neighbour)
                } else if !visited.contains(&neighbour) {
                    q.push(neighbour);
                }
            }
        }

        if ret.len() != 2 {
            panic!("found too many points");
        }
        let x_delta = ret[0].x.max(ret[1].x) - init.x;
        let crossing = ret[0].y != ret[1].y;

        return (crossing, x_delta);
    }

    fn count_inside(&self) -> i32 {
        let mut inside_count = 0;
        for y in 0..self.pipes.len() {
            let mut row_inside_count = 0;
            let mut inside = false;

            let mut x = 0;
            while x < self.pipes[y].len() {
                if self.pipes[y][x] != PipeType::GROUND {
                    let (crossing, x_delta) = self.get_start_and_end(Point {
                        y: y as i32,
                        x: x as i32,
                    });
                    if crossing {
                        inside = !inside;
                    }
                    x += x_delta as usize;
                } else if inside {
                    row_inside_count += 1;
                }

                x += 1;
            }
            inside_count += row_inside_count;
        }
        return inside_count;
    }

    fn print_map(&self, cur_point: Point) {
        let mut map_str = String::new();
        for y in 0..self.pipes.len() {
            for x in 0..self.pipes[y].len() {
                let p = Point {
                    y: y as i32,
                    x: x as i32,
                };
                if p == cur_point {
                    map_str = map_str + "#";
                } else {
                    map_str = map_str + &self.type_at(&p).to_string()
                }
            }
            map_str += "\n";
        }
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

#[aoc(day10, part2)]
fn day10part2(input: &str) -> i32 {
    let mut pipe_map = PipeMap::from_str_map(&input);
    pipe_map.mark_loop();

    pipe_map.print_map(Point { y: 0, x: 0 });

    let inside_count = pipe_map.count_inside();
    return inside_count;
}
