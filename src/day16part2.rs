#[derive(Copy, Clone, Debug, PartialEq)]
struct Coordinate {
    y: i32,
    x: i32,
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

#[derive(Copy, Clone, Debug, PartialEq)]
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
            Direction::LEFT => Coordinate { y: 0, x: -1 },
            Direction::RIGHT => Coordinate { y: 0, x: 1 },
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Empty,
    MirrorFromLeftGoesUp,
    MirrorFromLeftGoesDown,
    SplitVertical,
    SplitHorizontal,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '.' => Tile::Empty,
            '/' => Tile::MirrorFromLeftGoesUp,
            '\\' => Tile::MirrorFromLeftGoesDown,
            '|' => Tile::SplitVertical,
            '-' => Tile::SplitHorizontal,
            _ => panic!("Unknown tile type : {c}"),
        }
    }

    fn next_directions(&self, incoming_direction: Direction) -> [Direction; 2] {
        match self {
            Self::Empty => [incoming_direction, incoming_direction],
            Self::MirrorFromLeftGoesUp => match incoming_direction {
                Direction::UP => [Direction::RIGHT, Direction::RIGHT],
                Direction::RIGHT => [Direction::UP, Direction::UP],
                Direction::DOWN => [Direction::LEFT, Direction::LEFT],
                Direction::LEFT => [Direction::DOWN, Direction::DOWN],
            },
            Self::MirrorFromLeftGoesDown => match incoming_direction {
                Direction::UP => [Direction::LEFT, Direction::LEFT],
                Direction::RIGHT => [Direction::DOWN, Direction::DOWN],
                Direction::DOWN => [Direction::RIGHT, Direction::RIGHT],
                Direction::LEFT => [Direction::UP, Direction::UP],
            },
            Self::SplitVertical => match incoming_direction {
                Direction::UP => [incoming_direction, incoming_direction],
                Direction::DOWN => [incoming_direction, incoming_direction],
                Direction::LEFT => [Direction::UP, Direction::DOWN],
                Direction::RIGHT => [Direction::UP, Direction::DOWN],
            },
            Self::SplitHorizontal => match incoming_direction {
                Direction::LEFT => [incoming_direction, incoming_direction],
                Direction::RIGHT => [incoming_direction, incoming_direction],
                Direction::UP => [Direction::LEFT, Direction::RIGHT],
                Direction::DOWN => [Direction::LEFT, Direction::RIGHT],
            },
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct TileBeam {
    tile: Tile,
    beams: [bool; 4],
}

impl TileBeam {
    fn from_char(c: char) -> TileBeam {
        TileBeam {
            tile: Tile::from_char(c),
            beams: [false; 4],
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    tile_beam: Vec<Vec<TileBeam>>,
}

impl Grid {
    fn from_str(input: &str) -> Grid {
        Grid {
            tile_beam: input
                .trim()
                .lines()
                .map(|line| {
                    line.trim()
                        .chars()
                        .map(|c| TileBeam::from_char(c))
                        .collect()
                })
                .collect(),
        }
    }

    fn energize_init(&mut self) {
        self.energize(Coordinate { y: 0, x: 0 }, Direction::RIGHT)
    }

    fn get(&mut self, loc: Coordinate) -> &mut TileBeam {
        return &mut self.tile_beam[loc.y as usize][loc.x as usize];
    }

    fn try_move(&self, beam_loc: Coordinate, beam_type: Direction) -> Option<Coordinate> {
        let movement = beam_type.to_movement();

        let new_loc = Coordinate {
            y: beam_loc.y + movement.y,
            x: beam_loc.x + movement.x,
        };

        if new_loc.y >= 0
            && new_loc.x >= 0
            && (new_loc.y as usize) < self.tile_beam.len()
            && (new_loc.x as usize) < self.tile_beam[new_loc.y as usize].len()
        {
            return Some(new_loc);
        }
        return None;
    }

    fn energize(&mut self, beam_loc: Coordinate, beam_type: Direction) {
        // if this tile is already energized, terminate
        let tb = self.get(beam_loc);
        if tb.beams[beam_type as usize] {
            // already energized
            return;
        } else {
            tb.beams[beam_type as usize] = true;

            for new_direction in tb.tile.next_directions(beam_type) {
                let new_beam_loc = self.try_move(beam_loc, new_direction);
                match new_beam_loc {
                    Some(new_loc) => self.energize(new_loc, new_direction),
                    None => {}
                }
            }
        }
    }

    fn count_energy(&self) -> usize {
        let mut sum = 0;

        for tile_beam_row in &self.tile_beam {
            for tb in tile_beam_row {
                if tb.beams.iter().any(|b| *b) {
                    sum += 1;
                }
            }
        }
        return sum;
    }

    fn max_count_energy(&self) -> usize {
        let mut max_energy = usize::MIN;

        for y in 0..self.tile_beam.len() {
            for (direction, x) in [
                (Direction::RIGHT, 0),
                (Direction::LEFT, self.tile_beam[0].len() - 1),
            ] {
                let mut grid_copy = self.clone();
                grid_copy.energize(
                    Coordinate {
                        y: y as i32,
                        x: x as i32,
                    },
                    direction,
                );
                max_energy = max_energy.max(grid_copy.count_energy());
            }
        }
        for x in 0..self.tile_beam[0].len() {
            for (direction, y) in [
                (Direction::DOWN, 0),
                (Direction::UP, self.tile_beam.len() - 1),
            ] {
                let mut grid_copy = self.clone();
                grid_copy.energize(
                    Coordinate {
                        y: y as i32,
                        x: x as i32,
                    },
                    direction,
                );
                max_energy = max_energy.max(grid_copy.count_energy());
            }
        }

        return max_energy;
    }
}

#[aoc(day16, part2)]
fn day16part2(input: &str) -> usize {
    let grid = Grid::from_str(input);
    return grid.max_count_energy();
}
