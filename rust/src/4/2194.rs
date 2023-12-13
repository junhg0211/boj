use std::io::stdin;
use std::collections::{ HashSet, VecDeque };

fn can_go_up(
    y: usize, x: usize,
    unit_width: usize, obstacles: &HashSet<(usize, usize)>
) -> bool {
    if y <= 0 {
        return false;
    }

    for i in 0..unit_width {
        if obstacles.contains(&(y-1, x+i)) {
            return false;
        }
    }

    return true;
}

fn can_go_down(
    y: usize, x: usize, height: usize,
    unit_width: usize, unit_height: usize, obstacles: &HashSet<(usize, usize)>
) -> bool {
    if y+unit_height >= height {
        return false;
    }

    for i in 0..unit_width {
        if obstacles.contains(&(y+unit_height, x+i)) {
            return false;
        }
    }

    return true;
}

fn can_go_left(
    y: usize, x: usize,
    unit_height: usize, obstacles: &HashSet<(usize, usize)>
) -> bool {
    if x <= 0 {
        return false;
    }

    for i in 0..unit_height {
        if obstacles.contains(&(y+i, x-1)) {
            return false;
        }
    }

    return true;
}

fn can_go_right(
    y: usize, x: usize, width: usize,
    unit_width: usize, unit_height: usize, obstacles: &HashSet<(usize, usize)>
) -> bool {
    if x+unit_width >= width {
        return false;
    }

    for i in 0..unit_height {
        if obstacles.contains(&(y+i, x+unit_width)) {
            return false;
        }
    }

    return true;
}

fn main() {
    // -- get data
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let height = iter.next().unwrap();
    let width = iter.next().unwrap();
    let unit_height = iter.next().unwrap();
    let unit_width = iter.next().unwrap();
    let obstacle_count = iter.next().unwrap();

    let mut obstacles = HashSet::new();
    for _ in 0..obstacle_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());
        let y = iter.next().unwrap() - 1;
        let x = iter.next().unwrap() - 1;

        obstacles.insert((y, x));
    }

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let start_y = iter.next().unwrap() - 1;
    let start_x = iter.next().unwrap() - 1;

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let end_y = iter.next().unwrap() - 1;
    let end_x = iter.next().unwrap() - 1;

    // -- calculate distance
    let mut stack = VecDeque::new();
    let mut beens = HashSet::new();
    stack.push_back((start_y, start_x, 0));
    beens.insert((start_y, start_x));
    while let Some((y, x, distance)) = stack.pop_front() {
        if y == end_y && x == end_x {
            println!("{}", distance);
            return;
        }

        if can_go_up(y, x, unit_width, &obstacles)
        && !beens.contains(&(y-1, x)) {
            stack.push_back((y-1, x, distance+1));
            beens.insert((y-1, x));
        }

        if can_go_down(y, x, height, unit_width, unit_height, &obstacles)
        && !beens.contains(&(y+1, x)) {
            stack.push_back((y+1, x, distance+1));
            beens.insert((y+1, x));
        }

        if can_go_left(y, x, unit_height, &obstacles)
        && !beens.contains(&(y, x-1)) {
            stack.push_back((y, x-1, distance+1));
            beens.insert((y, x-1));
        }

        if can_go_right(y, x, width, unit_width, unit_height, &obstacles)
        && !beens.contains(&(y, x+1)) {
            stack.push_back((y, x+1, distance+1));
            beens.insert((y, x+1));
        }
    }

    println!("-1");
}
