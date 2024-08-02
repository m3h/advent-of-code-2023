#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
    z: usize,
}

impl Coordinate {
    fn set_down(&mut self) {
        self.z -= 1;
    }
    fn get_down(&self) -> Coordinate {
        let mut new_cube = *self;
        new_cube.set_down();
        return new_cube;
    }

    fn from_str(s: &str) -> Coordinate {
        let coordinates = s.split(",").map(|n| n.parse().unwrap()).collect::<Vec<_>>();

        return Coordinate {
            x: coordinates[0],
            y: coordinates[1],
            z: coordinates[2],
        };
    }
}

#[derive(Clone)]
struct Brick {
    cubes: Vec<Coordinate>,
}

impl Brick {
    fn from_str(s: &str) -> Brick {
        let start_and_end = s
            .split("~")
            .map(|c| Coordinate::from_str(c))
            .collect::<Vec<_>>();

        return Brick::from_fill(start_and_end[0], start_and_end[1]);
    }
    fn from_fill(c1: Coordinate, c2: Coordinate) -> Brick {
        let mut cubes = Vec::new();
        for x in (c1.x)..(c2.x + 1) {
            for y in (c1.y)..(c2.y + 1) {
                for z in (c1.z)..(c2.z + 1) {
                    cubes.push(Coordinate { x, y, z });
                }
            }
        }
        return Brick { cubes };
    }
}

#[derive(Clone)]
struct Stack {
    // position is brick ID
    bricks: Vec<Brick>,
    brick_map: Vec<Vec<Vec<usize>>>,
}

const NO_BRICK: usize = usize::MAX;
const GROUND: usize = usize::MAX - 1;
static ASCII_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

impl Stack {
    fn settle_pass(&mut self) -> bool {
        let mut movement = false;
        for brick_idx in 0..self.bricks.len() {
            if self.is_supported_by(brick_idx).len() == 0 {
                movement = true;
                for cube_idx in 0..self.bricks[brick_idx].cubes.len() {
                    let cube = self.bricks[brick_idx].cubes[cube_idx];
                    let new_cube = cube.get_down();

                    self.set_cube(cube, NO_BRICK);
                    self.set_cube(new_cube, brick_idx);
                    self.bricks[brick_idx].cubes[cube_idx] = new_cube;
                }
            }
        }
        return movement;
    }
    fn settle(&mut self) {
        while self.settle_pass() {}
    }

    fn non_supporting_bricks(&self) -> std::collections::HashSet<usize> {
        let mut non_supporting = std::collections::HashSet::new();

        for brick_idx in 0..self.bricks.len() {
            let mut jenga_stack = self.clone();
            for cube in jenga_stack.bricks[brick_idx].cubes.clone() {
                jenga_stack.set_cube(cube, NO_BRICK);
            }
            if !jenga_stack.clone().settle_pass() {
                non_supporting.insert(brick_idx);
            } else {
                for cube in jenga_stack.bricks[brick_idx].cubes.clone() {
                    jenga_stack.set_cube(cube, brick_idx);
                }
            }
        }

        return non_supporting;
    }
    fn is_supported_by(&self, brick_idx: usize) -> std::collections::HashSet<usize> {
        let mut bricks_below = std::collections::HashSet::new();

        for cube in &self.bricks[brick_idx].cubes {
            if cube.z == 1 {
                bricks_below.insert(GROUND);
            } else {
                let brick_below = self.get_cube(cube.get_down());
                if brick_below != brick_idx && brick_below != NO_BRICK {
                    bricks_below.insert(brick_below);
                }
            }
        }
        return bricks_below;
    }

    fn set_cube(&mut self, c: Coordinate, v: usize) {
        self.brick_map[c.x][c.y][c.z] = v;
    }

    fn get_cube(&self, c: Coordinate) -> usize {
        self.brick_map[c.x][c.y][c.z]
    }

    fn from_str(s: &str) -> Stack {
        let bricks = s
            .trim()
            .lines()
            .map(|line| Brick::from_str(line.trim()))
            .collect::<Vec<_>>();

        let mut x_max = 0;
        let mut y_max = 0;
        let mut z_max = 0;
        for brick in &bricks {
            for cube in &brick.cubes {
                x_max = x_max.max(cube.x);
                y_max = y_max.max(cube.y);
                z_max = z_max.max(cube.z);
            }
        }

        let mut brick_map = Vec::new();
        for x in 0..(x_max + 1) {
            brick_map.push(Vec::new());
            for y in 0..(y_max + 1) {
                brick_map[x].push(Vec::new());
                for z in 0..(z_max + 1) {
                    brick_map[x][y].push(NO_BRICK);
                }
            }
        }

        let mut stack = Stack { bricks, brick_map };
        for i in 0..stack.bricks.len() {
            for cube in stack.bricks[i].cubes.clone() {
                stack.set_cube(cube, i)
            }
        }

        return stack;
    }

    fn visualize(&self) {
        let mut x_view = String::new();
        for x in 0..self.brick_map.len() {
            x_view.push_str(&(x % 10).to_string());
        }
        x_view.push('\n');
        for z in (0..self.brick_map[0][0].len()).rev() {
            for x in 0..self.brick_map.len() {
                let mut brick_idx = NO_BRICK;
                for y in 0..self.brick_map[x].len() {
                    let found_brick = self.get_cube(Coordinate { x, y, z });
                    if brick_idx == NO_BRICK {
                        brick_idx = found_brick;
                    }
                }
                let brick_symbol = if z == 0 {
                    '-'
                } else if brick_idx == NO_BRICK {
                    '.'
                } else {
                    ASCII_UPPER[brick_idx % ASCII_UPPER.len()]
                };

                x_view.push(brick_symbol);
            }
            x_view.push(' ');
            x_view.push_str(&z.to_string());
            x_view.push('\n');
        }

        let mut y_view = String::new();
        for y in 0..self.brick_map[0].len() {
            y_view.push_str(&(y % 10).to_string());
        }
        y_view.push('\n');
        for z in (0..self.brick_map[0][0].len()).rev() {
            for y in 0..self.brick_map[0].len() {
                let mut brick_idx = NO_BRICK;
                for x in 0..self.brick_map.len() {
                    let found_brick = self.get_cube(Coordinate { x, y, z });
                    if brick_idx == NO_BRICK {
                        brick_idx = found_brick;
                    }
                }
                let brick_symbol = if z == 0 {
                    '-'
                } else if brick_idx == NO_BRICK {
                    '.'
                } else {
                    ASCII_UPPER[brick_idx % ASCII_UPPER.len()]
                };

                y_view.push(brick_symbol);
            }
            y_view.push(' ');
            y_view.push_str(&z.to_string());
            y_view.push('\n');
        }

        println!("x:\n{x_view}\n\ny:\n{y_view}");
    }
}

#[aoc(day22, part1)]
fn day22part1(input: &str) -> usize {
    // let input = "1,0,1~1,2,1
    // 0,0,2~2,0,2
    // 0,2,3~2,2,3
    // 0,0,4~0,2,4
    // 2,0,5~2,2,5
    // 0,1,6~2,1,6
    // 1,1,8~1,1,9";

    let mut stack = Stack::from_str(input);
    println!("init");
    stack.visualize();
    stack.settle();
    println!("settled");
    stack.visualize();
    return stack.non_supporting_bricks().len();
}
