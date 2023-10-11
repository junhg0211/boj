use std::io::stdin;
use std::collections::{ HashSet, VecDeque };

fn tick() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let board_size = input.trim().parse::<u32>().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    let now_x = iter.next().unwrap();
    let now_y = iter.next().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    let target = (
        iter.next().unwrap(),
        iter.next().unwrap(),
    );

    let mut queue = VecDeque::new();
    let mut beens = HashSet::new();

    queue.push_back((now_x, now_y, 0));
    beens.insert((now_x, now_y));
    let deltas = vec![
        (-2, -1),
        (-2, 1),
        (-1, -2),
        (-1, 2),
        (2, -1),
        (2, 1),
        (1, -2),
        (1, 2),
    ];
    loop {
        let (x, y, distance) = match queue.pop_front() {
            Some(now) => now,
            None => break,
        };

        if (x, y) == target {
            println!("{}", distance);
            return;
        }

        for (dx, dy) in &deltas {
            let (ax, ay) = ((x as i32 + dx) as u32, (y as i32 + dy) as u32);

            if ax < board_size && ay < board_size
                    && !beens.contains(&(ax, ay)) {
                queue.push_back((ax, ay, distance+1));
                beens.insert((ax, ay));
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let testcase_count = input.trim().parse::<usize>().unwrap();

    for _ in 0..testcase_count {
        tick();
    }
}
