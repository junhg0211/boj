use std::collections::HashMap;
use std::io::stdin;

fn ccw(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) -> i32 {
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

fn does_intersect(
    mut x1: i32,
    mut y1: i32,
    mut x2: i32,
    mut y2: i32,
    mut x3: i32,
    mut y3: i32,
    mut x4: i32,
    mut y4: i32,
) -> bool {
    let acb = ccw(x1, y1, x3, y3, x2, y2);
    let adb = ccw(x1, y1, x4, y4, x2, y2);

    if acb == adb && acb != 0 {
        return false;
    }

    if acb != adb {
        let cad = ccw(x3, y3, x1, y1, x4, y4);
        let cbd = ccw(x3, y3, x2, y2, x4, y4);

        if cad != cbd {
            return true;
        } else {
            return false;
        }
    }

    // -- if acb == adb && acb == 0 {

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
        return true;
    } else if (y1 != y2 || y3 != y4)
        && (y1 <= y3 && y3 <= y2
            || y1 <= y4 && y4 <= y2
            || y3 <= y1 && y1 <= y4
            || y3 <= y2 && y2 <= y4)
    {
        return true;
    } else {
        return false;
    }
}

fn get_ancient(original: usize, parent_map: &mut HashMap<usize, usize>) -> usize {
    let mut number = original;
    while let Some(thing) = parent_map.get(&number) {
        if &number == thing {
            break;
        }
        number = *thing;
    }

    parent_map.insert(original, number);
    return number;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let line_count = input.trim().parse::<usize>().unwrap();

    let mut parent = HashMap::new();
    let mut lines: Vec<Vec<i32>> = Vec::new();
    for i in 0..line_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let line = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        for (j, &ref aline) in lines.iter().enumerate() {
            if does_intersect(
                aline[0], aline[1], aline[2], aline[3], line[0], line[1], line[2], line[3],
            ) {
                let ancient = get_ancient(j, &mut parent);
                parent.insert(ancient, i);
            }
        }

        lines.push(line);
    }

    // println!("{:?}", parent);

    let mut count = HashMap::new();
    for i in 0..line_count {
        let ancient = get_ancient(i, &mut parent);
        count.insert(ancient, count.get(&ancient).unwrap_or(&0) + 1);
    }

    println!("{}\n{}", count.len(), count.values().max().unwrap());
}
