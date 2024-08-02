fn gcd(a: i128, b: i128) -> i128 {
    let a = a.abs();
    let b = b.abs();

    if b == 0 {
        return a;
    } else {
        let n_max = a.max(b);
        let n_min = a.min(b);
        return gcd(n_min, n_max % n_min);
        // return gcd(n_max - n_min, n_min);
    }
}

fn lcm(a: i128, b: i128) -> i128 {
    (a * b).abs() / gcd(a, b)
}
#[derive(Clone, Copy, PartialEq, Eq)]
struct Decimal {
    n: i128,
    d: i128,
}

impl std::cmp::PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::Ord for Decimal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_mag = self.n * other.d;
        let other_mag = other.n * self.d;

        return self_mag.cmp(&other_mag);
    }
}

impl Decimal {
    const ZERO: Decimal = Decimal { n: 0, d: 1 };
    const ONE: Decimal = Decimal { n: 1, d: 1 };

    fn simplify(&self) -> Self {
        let mut n = self.n;
        let mut d = self.d;

        if n == 0 {
            return Decimal::ZERO;
        }

        if n < 0 && d < 0 {
            n = n * -1;
            d = d * -1;
        } else if n < 0 || d < 0 {
            n = n.abs() * -1;
            d = d.abs();
        }
        // println!("gcd({n}, {d})");
        let nd_gcd = gcd(n, d);
        return Decimal {
            n: n / nd_gcd,
            d: d / nd_gcd,
        };
    }
}

impl From<i128> for Decimal {
    fn from(number: i128) -> Self {
        return Decimal { n: number, d: 1 };
    }
}

impl std::ops::Add<Decimal> for Decimal {
    type Output = Decimal;
    fn add(self, _rhs: Decimal) -> Self::Output {
        Decimal {
            n: self.n * _rhs.d + _rhs.n * self.d,
            d: self.d * _rhs.d,
        }
        .simplify()
    }
}

impl std::ops::Sub<Decimal> for Decimal {
    type Output = Decimal;

    fn sub(self, _rhs: Decimal) -> Self::Output {
        Decimal {
            n: self.n * _rhs.d - _rhs.n * self.d,
            d: self.d * _rhs.d,
        }
        .simplify()
    }
}

impl std::ops::Mul<Decimal> for Decimal {
    type Output = Decimal;

    fn mul(self, _rhs: Decimal) -> Self::Output {
        Decimal {
            n: self.n * _rhs.n,
            d: self.d * _rhs.d,
        }
        .simplify()
    }
}

impl std::ops::Div<Decimal> for Decimal {
    type Output = Decimal;

    fn div(self, _rhs: Decimal) -> Self::Output {
        if _rhs == Decimal::ZERO {
            panic!("Vision by zero!");
        }
        Decimal {
            n: self.n * _rhs.d,
            d: self.d * _rhs.n,
        }
        .simplify()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Coordinate {
    x: Decimal,
    y: Decimal,
    z: Decimal,
}

impl Coordinate {
    fn within(&self, p1: Coordinate, p2: Coordinate) -> bool {
        let x_min = p1.x.min(p2.x);
        let x_max = p1.x.max(p2.x);
        let y_min = p1.y.min(p2.y);
        let y_max = p1.y.max(p2.y);

        let c1 = self.x >= x_min;
        let c2 = self.x <= x_max;
        let c3 = self.y >= y_min;
        let c4 = self.y <= y_max;

        return c1 && c2 && c3 && c4;
    }

    fn unit(v: Decimal) -> Coordinate {
        return Coordinate { x: v, y: v, z: v };
    }
}

impl std::ops::Add<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn add(self, _rhs: Coordinate) -> Self::Output {
        return Coordinate {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        };
    }
}

impl std::ops::Sub<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn sub(self, _rhs: Coordinate) -> Self::Output {
        return Coordinate {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        };
    }
}

impl std::ops::Mul<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn mul(self, _rhs: Coordinate) -> Self::Output {
        return Coordinate {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        };
    }
}

impl std::ops::Div<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn div(self, _rhs: Coordinate) -> Self::Output {
        return Coordinate {
            x: self.x / _rhs.x,
            y: self.y / _rhs.y,
            z: self.z / _rhs.z,
        };
    }
}

impl std::fmt::Display for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = self.n.to_string();
        if self.d != 1 {
            s += "/";
            s += &self.d.to_string();
        }
        return f.write_str(&s);
    }
}

impl std::fmt::Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = "[".to_string();
        for n in &self.elements {
            s += &n.to_string();
            s += ",";
        }
        s += "]";
        return f.write_str(&s);
    }
}

#[derive(Clone)]
struct Matrix {
    elements: Vec<Decimal>,
}

impl Matrix {
    fn identity() -> Self {
        Matrix {
            elements: vec![Decimal::from(1)],
        }
    }

    fn broadcast_to(&self, target_size: usize) -> Self {
        if target_size % self.elements.len() != 0 {
            panic!("incorrect broadcast size");
        }
        let multiplier = target_size / self.elements.len();

        let mut new_elements = vec![];
        for el in &self.elements {
            for _ in 0..multiplier {
                new_elements.push(el.clone());
            }
        }
        return Matrix {
            elements: new_elements,
        };
    }
    fn broadcast(&self, other: &Self) -> (Self, Self) {
        if self.elements.len() == other.elements.len() {
            return (self.clone(), other.clone());
        } else if self.elements.len() < other.elements.len() {
            return (self.broadcast_to(other.elements.len()), other.clone());
        } else {
            return (self.clone(), other.broadcast_to(self.elements.len()));
        }
    }
}

impl std::ops::Add<Matrix> for Matrix {
    type Output = Matrix;
    fn add(self, _rhs: Matrix) -> Self::Output {
        let new_size = self.elements.len().max(_rhs.elements.len());
        let lhs = self.broadcast_to(new_size);
        let rhs = _rhs.broadcast_to(new_size);
        Matrix {
            elements: (0..new_size)
                .map(|i| lhs.elements[i] + rhs.elements[i])
                .collect(),
        }
    }
}

impl std::ops::Sub<Matrix> for Matrix {
    type Output = Matrix;
    fn sub(self, _rhs: Matrix) -> Self::Output {
        let new_size = self.elements.len().max(_rhs.elements.len());
        let lhs = self.broadcast_to(new_size);
        let rhs = _rhs.broadcast_to(new_size);
        Matrix {
            elements: (0..new_size)
                .map(|i| lhs.elements[i] - rhs.elements[i])
                .collect(),
        }
    }
}
impl std::ops::Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, _rhs: Matrix) -> Self::Output {
        let new_size = self.elements.len().max(_rhs.elements.len());
        let lhs = self.broadcast_to(new_size);
        let rhs = _rhs.broadcast_to(new_size);
        Matrix {
            elements: (0..new_size)
                .map(|i| lhs.elements[i] * rhs.elements[i])
                .collect(),
        }
    }
}

impl std::ops::Div<Matrix> for Matrix {
    type Output = Matrix;
    fn div(self, _rhs: Matrix) -> Self::Output {
        let new_size = self.elements.len().max(_rhs.elements.len());
        let lhs = self.broadcast_to(new_size);
        let rhs = _rhs.broadcast_to(new_size);
        Matrix {
            elements: (0..new_size)
                .map(|i| lhs.elements[i] / rhs.elements[i])
                .collect(),
        }
    }
}

fn error(rock_p: &Matrix, rock_v: &Matrix, p: &Matrix, v: &Matrix, t: &Matrix) -> Matrix {
    let hail_position = p.clone() + v.clone() * t.clone();
    let rock_position = rock_p.clone() + rock_v.clone() * t.clone();

    return rock_position - hail_position;
}

fn gradient_descent(p: Matrix, v: Matrix) -> Matrix {
    let mut t = Matrix::identity().broadcast_to(p.elements.len() / 3);

    let mut rock_p = Matrix::identity().broadcast_to(3);
    let mut rock_v = Matrix::identity().broadcast_to(3);

    let rock_p_n1 = rock_p.clone() - Matrix::identity();
    let rock_v_n1 = rock_v.clone() - Matrix::identity();
    let t_n1 = t.clone() - Matrix::identity();

    let f = error(&rock_p, &rock_v, &p, &p, &t);
    let f_n1 = error(&rock_p_n1, &rock_v_n1, &p, &p, &t_n1);

    let gradient = f - f_n1;

    return gradient;
}

#[aoc(day24, part2)]
fn day24part2(input: &str) -> usize {
    let input = "19, 13, 30 @ -2,  1, -2
    18, 19, 22 @ -1, -1, -2
    20, 25, 34 @ -2, -2, -4
    12, 31, 28 @ -1, -2, -1
    20, 19, 15 @  1, -5, -3";

    let mut p = vec![];
    let mut v = vec![];

    for line in input.trim().lines() {
        let line = line.trim().split(" @ ").collect::<Vec<_>>();
        assert!(line.len() == 2);

        let p_and_v = line
            .iter()
            .map(|nums| {
                nums.split(", ")
                    .map(|n| Decimal::from(n.trim().parse::<i128>().unwrap()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        assert!(p_and_v.len() == 2);

        p.extend(p_and_v[0].clone());
        v.extend(p_and_v[1].clone());
    }

    let p = Matrix { elements: p };
    let v = Matrix { elements: v };

    let t = Matrix {
        elements: vec![Decimal::from(5)],
    };

    let a = gradient_descent(p, v);

    return 5;
}
