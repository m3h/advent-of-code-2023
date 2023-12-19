#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn from_int(i: usize) -> Direction {
        match i {
            0 => Direction::RIGHT,
            1 => Direction::DOWN,
            2 => Direction::LEFT,
            3 => Direction::UP,
            _ => panic!("Unknown Direction int!"),
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

enum CornerType {
    INNER,
    OUTER,
}
struct AreaCalculator {
    pos: Coordinate,
    dir: Direction,
    double_area: i64,
    border_straight_quarters: i64,
    border_corners_inner: i64,
    border_corners_outer: i64,
    initial_pos: Coordinate,
    initial_dir: Direction,
    initial_set: bool,
}
impl AreaCalculator {
    fn new() -> AreaCalculator {
        let initial_pos = Coordinate { y: 0, x: 0 };
        let initial_dir = Direction::RIGHT;
        return AreaCalculator {
            initial_set: false,
            initial_pos,
            initial_dir,
            pos: initial_pos,
            dir: initial_dir,
            double_area: 0,
            border_corners_inner: 0,
            border_corners_outer: 0,
            border_straight_quarters: 0,
        };
    }

    fn corner_type(from: Direction, to: Direction) -> CornerType {
        match from {
            Direction::UP => match to {
                Direction::UP => panic!("not sure how this is handled"),
                Direction::RIGHT => CornerType::OUTER,
                Direction::DOWN => panic!("not sure how this is handled"),
                Direction::LEFT => CornerType::INNER,
            },
            Direction::RIGHT => match to {
                Direction::RIGHT => panic!("not sure how this is handled"),
                Direction::DOWN => CornerType::OUTER,
                Direction::LEFT => panic!("not sure how this is handled"),
                Direction::UP => CornerType::INNER,
            },
            Direction::DOWN => match to {
                Direction::DOWN => panic!("not sure how this is handled"),
                Direction::LEFT => CornerType::OUTER,
                Direction::UP => panic!("not sure how this is handled"),
                Direction::RIGHT => CornerType::INNER,
            },
            Direction::LEFT => match to {
                Direction::LEFT => panic!("not sure how this is handled"),
                Direction::UP => CornerType::OUTER,
                Direction::RIGHT => panic!("not sure how this is handled"),
                Direction::DOWN => CornerType::INNER,
            },
        }
    }

    fn from_str(input: &str) -> i64 {
        let mut area_calculator = AreaCalculator::new();

        for (direction, steps) in input.trim().lines().map(|line| {
            let line_parts: Vec<&str> = line.split_whitespace().collect();

            let hex_str = line_parts[2].trim_start_matches("(#").trim_end_matches(")");
            let steps_str = &hex_str[..hex_str.len() - 1];
            let direction_hex_digit = &hex_str[hex_str.len() - 1..];

            let steps = usize::from_str_radix(steps_str, 16).unwrap();
            let direction = usize::from_str_radix(direction_hex_digit, 16).unwrap();
            let direction = Direction::from_int(direction);
            return (direction, steps);
        }) {
            area_calculator.add_vertex(direction, steps);
        }

        return area_calculator.finalize_area();
    }

    fn init_start_if_unset(&mut self, pos: Coordinate, dir: Direction) -> bool {
        if !self.initial_set {
            self.initial_pos = pos;
            self.initial_dir = dir;
            self.pos = pos;
            self.dir = dir;
            self.initial_set = true;
            return true;
        }
        return false;
    }
    fn add_vertex(&mut self, direction: Direction, steps: usize) {
        let initialized = self.init_start_if_unset(Coordinate { y: 0, x: 0 }, direction);
        // use shoelaces to calculate resultant area as we process vertices
        // https://en.wikipedia.org/wiki/Shoelace_formula#Trapezoid_formula
        let unit_movement = direction.to_movement();

        let p1 = self.pos;
        let p2 = Coordinate {
            y: p1.y + unit_movement.y * (steps as i64),
            x: p1.x + unit_movement.x * (steps as i64),
        };

        self.double_area += (p1.y + p2.y) * (p1.x - p2.x);

        self.border_straight_quarters += ((steps as i64) + 1) * 2;

        if !initialized {
            let corner_type = AreaCalculator::corner_type(self.dir, direction);
            match corner_type {
                CornerType::OUTER => self.border_corners_outer += 1,
                CornerType::INNER => self.border_corners_inner += 1,
            }
        }
        self.pos = p2;
        self.dir = direction;
    }

    fn finalize_area(&mut self) -> i64 {
        // we assume the elves know how to dig a nicely formed polygon

        // the last corner is a bit trickier, as we only only both sides at
        // the end of the polygon
        let corner_type = AreaCalculator::corner_type(self.dir, self.initial_dir);
        match corner_type {
            CornerType::INNER => self.border_corners_inner += 1,
            CornerType::OUTER => self.border_corners_outer += 1,
        }

        assert!(self.pos == self.initial_pos);

        assert!(self.double_area % 2 == 0);

        if self.double_area < 0 {
            // negative area means we took the other way round the shape
            self.double_area = -self.double_area;
            let tmp_inner = self.border_corners_inner;
            self.border_corners_inner = self.border_corners_outer;
            self.border_corners_outer = tmp_inner;
        }

        let quarter_sum = self.border_straight_quarters
            - self.border_corners_inner * 3
            - self.border_corners_outer;
        assert!(quarter_sum % 4 == 0);

        let area = self.double_area / 2 + quarter_sum / 4;
        return area;
    }
}
#[aoc(day18, part2)]
fn day18part2(input: &str) -> i64 {
    // inner area = 24
    // innert border area = 74 / 4 = 37
    // straight border 52 / 2 = 26
    // 9 outer corners = -(9*1) / 4
    // 5 inner corners = -(5*3) / 4
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

    // inner area = 1
    // straight border = 16 / 4 = 4
    // outer corners 4  = -(4*1) / 4 = -1
    // let input = "R 1 (#70c710)
    // D 1 (#0dc571)
    // L 1 (#5713f0)
    // U 1 (#d2c081)";

    return AreaCalculator::from_str(input);
}
