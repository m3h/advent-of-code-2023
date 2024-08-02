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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug)]
struct Hailstone {
    p: Coordinate,
    v: Coordinate,
}

impl From<&str> for Hailstone {
    fn from(description: &str) -> Self {
        let p_and_v = description
            .split(" @ ")
            .map(|coord| {
                coord
                    .split(", ")
                    .map(|n| Decimal::from(n.trim().parse::<i128>().unwrap()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let p = p_and_v.get(0).unwrap();
        let v = p_and_v.get(1).unwrap();

        let p = Coordinate {
            x: p[0],
            y: p[1],
            z: p[2],
        };
        let v = Coordinate {
            x: v[0],
            y: v[1],
            z: v[2],
        };
        return Hailstone { p, v };
    }
}

fn intersect(p1: Coordinate, v1: Coordinate, p2: Coordinate, v2: Coordinate) -> Option<Coordinate> {
    // check for division by zero
    if v2.x == Decimal::ZERO || v1.x == Decimal::ZERO {
        return None;
    }
    let div = (v1.y / v1.x - v2.y / v2.x);
    if div == Decimal::ZERO {
        return None;
    }
    let o_x = (p2.y - v2.y * p2.x / v2.x - p1.y + v1.y * p1.x / v1.x) / div;

    let t1 = (o_x - p1.x) / v1.x;
    let t2 = (o_x - p2.x) / v2.x;

    if t1 < Decimal::ZERO || t2 < Decimal::ZERO {
        return None;
    }

    let t1 = Coordinate {
        x: t1,
        y: t1,
        z: t1,
    };
    let t2 = Coordinate {
        x: t2,
        y: t2,
        z: t2,
    };

    let o1 = p1 + t1 * v1;
    let o2 = p2 + t2 * v2;

    assert!(o1.x == o2.x);
    assert!(o1.y == o2.y);
    return Some(o1);
}

impl Hailstone {
    fn hits_at(&self, other: &Hailstone) -> Option<Coordinate> {
        return intersect(self.p, self.v, other.p, other.v);
    }
}
#[aoc(day24, part1)]
fn day24part1(input: &str) -> usize {
    // let input = "19, 13, 30 @ -2,  1, -2
    // 18, 19, 22 @ -1, -1, -2
    // 20, 25, 34 @ -2, -2, -4
    // 12, 31, 28 @ -1, -2, -1
    // 20, 19, 15 @  1, -5, -3";

    // x = x0 + vx * t = x0r + vxr * t
    // y = y0 + vy * t = y0r + vyr * t
    // z = z0 + vz * t = z0r + vzr * t

    // (vx - vxr) * t = x0r - x0
    // (vy - vyr) * t = y0r - y0
    // (vz - vzr) * t = z0r - z0

    // x1 = x0_1 + vx_1 * t = x0r + vxr * t
    // x2 = x0_2 + vx_2 * t = x0r + vxr * t

    // (vx_1 - vxr) * t = x0r - x0_1
    // (vx_2 - vxr) * t = x0r - x0_2

    // t = x0r - x0_1 / (vx_1 - vxr)
    // t = x0r - x0_2 / (vx_2 - vxr)

    let hailstones = input
        .trim()
        .lines()
        .map(|line| Hailstone::from(line.trim()))
        .collect::<Vec<_>>();

    let bound1 = 7;
    let bound2 = 27;

    let bound1 = 200000000000000;
    let bound2 = 400000000000000;

    let bound1 = Decimal::from(bound1);
    let bound2 = Decimal::from(bound2);

    let bound1 = Coordinate {
        x: bound1,
        y: bound1,
        z: bound1,
    };
    let bound2 = Coordinate {
        x: bound2,
        y: bound2,
        z: bound2,
    };

    let mut intersected_count = 0;
    for i in 0..hailstones.len() {
        for j in (i + 1)..hailstones.len() {
            let a = &hailstones[i];
            let b = &hailstones[j];

            match a.hits_at(b) {
                Some(intersection_point) => {
                    if intersection_point.within(bound1, bound2) {
                        intersected_count += 1
                    }
                }
                None => {}
            }
        }
    }

    return intersected_count;
}
