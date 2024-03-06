use std::cmp::Ordering;
use std::io::stdin;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug)]
struct Fraction {
    a: i64,
    b: i64,
}

fn get_gcd(mut a: i64, mut b: i64) -> i64 {
    if a < b {
        return get_gcd(b, a);
    }

    while b > 0 {
        (a, b) = (b, a % b);
    }

    a
}

impl Fraction {
    fn new(number: i64) -> Self {
        Fraction { a: number, b: 1 }
    }

    fn cleanup(&self) -> Self {
        let gcd = get_gcd(self.a.abs(), self.b.abs());
        let mut a = self.a / gcd;
        let mut b = self.b / gcd;

        if b < 0 {
            a *= -1;
            b *= -1;
        }

        // println!("{} {}", a, b);
        Fraction { a, b }
    }

    fn reduction(&self, other: &Self) -> (Self, Self) {
        let mut a = self.cleanup();
        let mut b = other.cleanup();

        let mut gcd = get_gcd(a.b.abs(), b.b.abs());
        let mut lcm = a.b * b.b / gcd;

        if lcm < 0 {
            gcd *= -1;
            lcm *= -1;
        }

        let ab = a.b;
        let bb = b.b;

        a.a = a.a * bb / gcd;
        a.b = lcm;
        b.a = b.a * ab / gcd;
        b.b = lcm;

        (a, b)
    }
}

impl PartialEq for Fraction {
    fn eq(&self, rhs: &Self) -> bool {
        let a = self.cleanup();
        let b = rhs.cleanup();

        a.a == b.a && a.b == b.b
    }
}

impl PartialOrd for Fraction {
    fn lt(&self, rhs: &Self) -> bool {
        let (a, b) = self.reduction(rhs);
        a.a < b.a
    }

    fn le(&self, rhs: &Self) -> bool {
        let (a, b) = self.reduction(rhs);
        a.a <= b.a
    }

    fn gt(&self, rhs: &Self) -> bool {
        let (a, b) = self.reduction(rhs);
        a.a > b.a
    }

    fn ge(&self, rhs: &Self) -> bool {
        let (a, b) = self.reduction(rhs);
        a.a >= b.a
    }

    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        if self < rhs {
            return Some(Ordering::Less);
        }

        if self > rhs {
            return Some(Ordering::Greater);
        }

        return Some(Ordering::Equal);
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let (a, b) = self.reduction(&rhs);

        Fraction {
            a: a.a - b.a,
            b: a.b,
        }
    }
}

impl Div for Fraction {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let a = self.cleanup();
        let b = rhs.cleanup();
        Fraction {
            a: a.a * b.b,
            b: a.b * b.a,
        }
        .cleanup()
    }
}

impl Neg for Fraction {
    type Output = Self;

    fn neg(self) -> Self {
        Fraction {
            a: -self.a,
            b: self.b,
        }
        .cleanup()
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let a = self.cleanup();
        let b = rhs.cleanup();
        Fraction {
            a: a.a * b.a,
            b: a.b * b.b,
        }
        .cleanup()
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let (a, b) = self.reduction(&rhs);

        Fraction {
            a: a.a + b.a,
            b: a.b,
        }
        .cleanup()
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap());

    let mut x1 = Fraction::new(iter.next().unwrap());
    let mut y1 = Fraction::new(iter.next().unwrap());
    let mut x2 = Fraction::new(iter.next().unwrap());
    let mut y2 = Fraction::new(iter.next().unwrap());

    if x1 > x2 {
        (x1, y1, x2, y2) = (x2, y2, x1, y1);
    }

    let mut t1 = (y2 - y1) / (x2 - x1);

    // l2
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap());

    let mut x3 = Fraction::new(iter.next().unwrap());
    let mut y3 = Fraction::new(iter.next().unwrap());
    let mut x4 = Fraction::new(iter.next().unwrap());
    let mut y4 = Fraction::new(iter.next().unwrap());

    if x3 > x4 {
        (x3, y3, x4, y4) = (x4, y4, x3, y3);
    }

    let mut t2 = (y4 - y3) / (x4 - x3);

    //
    if x3 == x4 {
        (x1, y1, x2, y2, x3, y3, x4, y4) = (x3, y3, x4, y4, x1, y1, x2, y2);
        (t1, t2) = (t2, t1);
    }

    // --- if l1 is ppdclr to yx
    if x1 == x2 {
        (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        // if l2 is ppdclr to xx
        if y3 == y4 {
            // if y3 in and x1 in
            if (y1 <= y3 && y3 <= y2) && (x3 <= x1 && x1 <= x4) {
                return println!("1");
            }
            return println!("0");
        }

        // if l2 is ppdclr to yx
        if x3 == x4 {
            (y3, y4) = if y3 < y4 { (y3, y4) } else { (y4, y3) };
            if (x1 == x3)
                && (y1 <= y3 && y3 <= y2
                    || y1 <= y4 && y4 <= y2
                    || y3 <= y1 && y1 <= y4
                    || y3 <= y2 && y2 <= y4)
            {
                return println!("1");
            }
            return println!("0");
        }

        let y = t2 * (x1 - x3) + y3;
        if (y1 <= y && y <= y2) && (x3 <= x1 && x1 <= x4) {
            return println!("1");
        }

        return println!("0");
    }

    // -- if ppdclr
    if t1 == t2 {
        // - if not on same line
        if -t1 * x1 + y1 != -t2 * x3 + y3 {
            return println!("0");
        }

        // - if on same line
        // if duplicate
        if x1 <= x3 && x3 <= x2
            || x1 <= x4 && x4 <= x2
            || x3 <= x1 && x1 <= x4
            || x3 <= x2 && x2 <= x4
        {
            return println!("1");
        }

        // if not duplicate
        return println!("0");
    }

    // -- if not ppdclr
    let x = ((t1 * x1 - t2 * x3) - (y1 - y3)) / (t1 - t2);

    let a = x1 <= x && x <= x2;
    let b = x3 <= x && x <= x4;
    if a && b {
        return println!("1");
    }

    return println!("0");
}
