use std::io::stdin;

#[test]
fn test_1() {
    assert_eq!(tick(1, 1, 5, 5, 1, 5, 5, 1), 1);
}
#[test]
fn test_2() {
    assert_eq!(tick(1, 1, 5, 5, 6, 10, 10, 6), 0);
}
#[test]
fn test_3() {
    assert_eq!(tick(1, 1, 5, 5, 5, 5, 1, 1), 1);
}
#[test]
fn test_4() {
    assert_eq!(tick(1, 1, 5, 5, 3, 3, 5, 5), 1);
}
#[test]
fn test_5() {
    assert_eq!(tick(1, 1, 5, 5, 3, 3, 1, 3), 1);
}
#[test]
fn test_6() {
    assert_eq!(tick(1, 1, 5, 5, 5, 5, 9, 9), 1);
}
#[test]
fn test_7() {
    assert_eq!(tick(1, 1, 5, 5, 6, 6, 9, 9), 0);
}
#[test]
fn test_8() {
    assert_eq!(tick(1, 1, 5, 5, 5, 5, 1, 5), 1);
}
#[test]
fn test_9() {
    assert_eq!(tick(1, 1, 5, 5, 6, 6, 1, 5), 0);
}
#[test]
fn test_10() {
    assert_eq!(tick(9, 3, 4, 3, 1, 3, 3, 3), 0);
}
#[test]
fn test_11() {
    assert_eq!(tick(2, 0, 0, 1, 1, 0, 1, 1), 1);
}
#[test]
fn test_12() {
    assert_eq!(tick(0, 0, 0, 1, 0, 2, 0, 3), 0);
}
#[test]
fn test_13() {
    assert_eq!(tick(0, 7, 7, 0, 4, 0, 4, 4), 1);
}
#[test]
fn test_14() {
    assert_eq!(tick(0, 1, 0, 2, 0, 3, 0, 4), 0);
}
#[test]
fn test_15() {
    assert_eq!(tick(0, 0, 0, 5, -1, 6, 1, 4), 1);
}
#[test]
fn test_16() {
    assert_eq!(tick(0, 0, 0, 6, -1, 7, 7, -1), 1);
}
#[test]
fn test_17() {
    assert_eq!(tick(1, 1, 1, 2, 1, 3, 3, 3), 0);
}
#[test]
fn test_18() {
    assert_eq!(tick(0, 0, 1, -1, 1, -1, 2, -2), 1);
}

fn ccw(x1: i64, y1: i64, x2: i64, y2: i64, x3: i64, y3: i64) -> i64 {
    let ax = x2 - x1;
    let ay = y2 - y1;
    let bx = x3 - x2;
    let by = y3 - y2;

    let cross = ax * by - ay * bx;

    if cross > 0 {
        return 1;
    }
    if cross < 0 {
        return -1;
    }
    return 0;
}

fn tick(
    mut x1: i64,
    mut y1: i64,
    mut x2: i64,
    mut y2: i64,
    mut x3: i64,
    mut y3: i64,
    mut x4: i64,
    mut y4: i64,
) -> i64 {
    let acb = ccw(x1, y1, x3, y3, x2, y2);
    let adb = ccw(x1, y1, x4, y4, x2, y2);

    if acb == adb && acb != 0 {
        // println!("D");
        return 0;
    }

    if acb != adb {
        let cad = ccw(x3, y3, x1, y1, x4, y4);
        let cbd = ccw(x3, y3, x2, y2, x4, y4);

        if cad != cbd {
            // println!("A");
            return 1;
        } else {
            // println!("E");
            return 0;
        }
    }

    // if acb == adb && acb == 0 {
    if x1 > x2 {
        (x1, y1, x2, y2) = (x2, y2, x1, y1);
    }
    if x3 > x4 {
        (x3, y3, x4, y4) = (x4, y4, x3, y3);
    }

    if (x1 != x2 || x3 != x4)
        && (x1 <= x3 && x3 <= x2
            || x1 <= x4 && x4 <= x2
            || x3 <= x1 && x1 <= x4
            || x3 <= x2 && x2 <= x4)
    {
        // println!("B");
        return 1;
    } else if (y1 != y2 || y3 != y4)
        && (y1 <= y3 && y3 <= y2
            || y1 <= y4 && y4 <= y2
            || y3 <= y1 && y1 <= y4
            || y3 <= y2 && y2 <= y4)
    {
        // println!("C");
        return 1;
    } else {
        // println!("F");
        return 0;
    }
    // }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap());

    let x1 = iter.next().unwrap();
    let y1 = iter.next().unwrap();
    let x2 = iter.next().unwrap();
    let y2 = iter.next().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap());

    let x3 = iter.next().unwrap();
    let y3 = iter.next().unwrap();
    let x4 = iter.next().unwrap();
    let y4 = iter.next().unwrap();

    println!("{}", tick(x1, y1, x2, y2, x3, y3, x4, y4));
}
