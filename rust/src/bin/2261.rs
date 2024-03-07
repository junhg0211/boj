use std::io::stdin;

#[test]
fn test_1() {
    let mut tree = QuadTree::new(-10000, -10000, 10001, 10001);
    tree.push(Point { id: 1, x: 0, y: 0 });
    tree.push(Point {
        id: 2,
        x: 10,
        y: 10,
    });
    tree.push(Point { id: 3, x: 0, y: 10 });
    tree.push(Point { id: 4, x: 10, y: 0 });

    assert_eq!(tree.tick(&tree), 100);
}

#[test]
fn test_2() {
    let mut tree = QuadTree::new(-10000, -10000, 10001, 10001);
    tree.push(Point { id: 1, x: 0, y: 0 });
    tree.push(Point {
        id: 2,
        x: 10000,
        y: 10000,
    });
    tree.push(Point {
        id: 3,
        x: -10000,
        y: -10000,
    });
    tree.push(Point { id: 4, x: 0, y: 0 });

    assert_eq!(tree.tick(&tree), 0);
}

#[test]
fn test_3() {
    let mut tree = QuadTree::new(-10000, -10000, 10001, 10001);
    tree.push(Point {
        id: 1,
        x: -1000,
        y: -1000,
    });
    tree.push(Point {
        id: 2,
        x: 1000,
        y: -1000,
    });
    tree.push(Point {
        id: 3,
        x: 1000,
        y: -10,
    });

    assert_eq!(tree.tick(&tree), 980100);
}

#[derive(Debug)]
struct Point {
    id: usize,
    x: i32,
    y: i32,
}

impl Point {
    fn distance_square(&self, other: &Point) -> i32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;

        return dx * dx + dy * dy;
    }
}

#[derive(Debug)]
struct QuadTree {
    divided: bool,
    points: Vec<Point>,
    subtree: Vec<QuadTree>,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

const MAX_COUNT: usize = 2;

impl QuadTree {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        QuadTree {
            divided: false,
            points: Vec::new(),
            subtree: Vec::new(),
            x1,
            y1,
            x2,
            y2,
        }
    }

    fn x_divisor(&self) -> i32 {
        return (self.x1 + self.x2) / 2;
    }

    fn y_divisor(&self) -> i32 {
        return (self.y1 + self.y2) / 2;
    }

    fn contains(&self, point: &Point) -> bool {
        return self.x1 <= point.x && point.x < self.x2 && self.y1 <= point.y && point.y < self.y2;
    }

    fn push(&mut self, point: Point) {
        if self.divided {
            for i in 0..self.subtree.len() {
                let subtree = &mut self.subtree[i];
                if subtree.contains(&point) {
                    subtree.push(point);
                    break;
                }
            }

            return;
        }

        if self.points.len() >= MAX_COUNT {
            self.divide();
            self.push(point);
            return;
        }

        self.points.push(point);
    }

    fn divide(&mut self) {
        let xd = self.x_divisor();
        let yd = self.y_divisor();

        if self.x2 - self.x1 >= 2 {
            self.subtree.push(QuadTree::new(self.x1, self.y1, xd, yd));
            self.subtree.push(QuadTree::new(xd, self.y1, self.x2, yd));
            self.subtree.push(QuadTree::new(self.x1, yd, xd, self.y2));
            self.subtree.push(QuadTree::new(xd, yd, self.x2, self.y2));
        }

        self.divided = true;

        while let Some(thing) = self.points.pop() {
            self.push(thing);
        }
    }

    fn get(&self, point: &Point) -> Option<&QuadTree> {
        if !self.contains(&point) {
            return None;
        }

        if !self.divided {
            return Some(self);
        }

        for i in 0..self.subtree.len() {
            let tree = &self.subtree[i];
            if self.contains(&point) {
                return tree.get(point);
            }
        }

        return None;
    }

    fn tick(&self, ancient: &QuadTree) -> i32 {
        if self.divided {
            // println!("S");
            let mut result = i32::MAX;
            for i in 0..self.subtree.len() {
                let value = self.subtree[i].tick(&self);
                if value < result {
                    result = value;
                }
            }

            return result;
        }

        let mut min_distance = i32::MAX;
        for point in self.points.iter() {
            let mut result = i32::MAX;

            // self
            if let Some(qt) = ancient.get(&point) {
                // println!("O");
                for other in qt.points.iter() {
                    if other.id == point.id {
                        continue;
                    }

                    println!("{} {}", other.id, point.id);

                    let distance = point.distance_square(&other);
                    if distance < result {
                        result = distance;
                    }
                }
            }

            // left
            if let Some(qt) = ancient.get(&Point {
                id: 0,
                x: self.x1 - 1,
                y: point.y,
            }) {
                // println!("A");
                for other in qt.points.iter() {
                    println!("{} {}", other.id, point.id);
                    let distance = point.distance_square(&other);
                    if distance < result {
                        result = distance;
                    }
                }
            }

            // right
            if let Some(qt) = ancient.get(&Point {
                id: 0,
                x: self.x2,
                y: point.y,
            }) {
                // println!("B");
                for other in qt.points.iter() {
                    println!("{} {}", other.id, point.id);
                    let distance = point.distance_square(&other);
                    if distance < result {
                        result = distance;
                    }
                }
            }

            // up
            if let Some(qt) = ancient.get(&Point {
                id: 0,
                x: point.x,
                y: self.y1 - 1,
            }) {
                // println!("C");
                for other in qt.points.iter() {
                    println!("{} {}", other.id, point.id);
                    let distance = point.distance_square(&other);
                    if distance < result {
                        result = distance;
                    }
                }
            }

            // down
            if let Some(qt) = ancient.get(&Point {
                id: 0,
                x: point.x,
                y: self.y2,
            }) {
                // println!("D");
                for other in qt.points.iter() {
                    println!("{} {}", other.id, point.id);
                    let distance = point.distance_square(&other);
                    if distance < result {
                        result = distance;
                    }
                }
            }

            if result < min_distance {
                min_distance = result;
            }
        }

        return min_distance;
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let point_count = input.trim().parse::<usize>().unwrap();

    let mut tree = QuadTree::new(-10000, -10000, 10001, 10001);
    for i in 1..=point_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());

        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        tree.push(Point { id: i, x, y });
    }

    println!("{}", tree.tick(&tree));
}
