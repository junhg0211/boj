use std::io::stdin;
use std::collections::{ VecDeque, HashSet };

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let height = iter.next().unwrap();
    let width = iter.next().unwrap();
    let mut board = Vec::new();
    for _ in 0..height {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let row = input.trim().chars().collect::<Vec<_>>();
        board.push(row);
    }

    // -- bfs
    let mut queue = VecDeque::new();
    let mut beens = HashSet::new();
    queue.push_back((0, 0, true, 1));
    beens.insert((0, 0, true));
    loop {
        let (y, x, breakable, distance) = match queue.pop_front() {
            Some(thing) => thing,
            None => break,
        };

        if x == width-1 && y == height-1 {
            println!("{}", distance);
            return;
        }

        // -- just move
        if y > 0 && board[y-1][x] == '0' && !beens.contains(&(y-1, x, breakable)) {
            queue.push_back((y-1, x, breakable, distance+1));
            beens.insert((y-1, x, breakable));
        }
        if x > 0 && board[y][x-1] == '0' && !beens.contains(&(y, x-1, breakable)) {
            queue.push_back((y, x-1, breakable, distance+1));
            beens.insert((y, x-1, breakable));
        }
        if y < height-1 && board[y+1][x] == '0' && !beens.contains(&(y+1, x, breakable)) {
            queue.push_back((y+1, x, breakable, distance+1));
            beens.insert((y+1, x, breakable));
        }
        if x < width-1 && board[y][x+1] == '0' && !beens.contains(&(y, x+1, breakable)) {
            queue.push_back((y, x+1, breakable, distance+1));
            beens.insert((y, x+1, breakable));
        }

        // -- break and move
        if !breakable {
            continue;
        }

        if y > 0 && board[y-1][x] == '1' && !beens.contains(&(y-1, x, false)) {
            queue.push_back((y-1, x, false, distance+1));
            beens.insert((y-1, x, false));
        }
        if x > 0 && board[y][x-1] == '1' && !beens.contains(&(y, x-1, false)) {
            queue.push_back((y, x-1, false, distance+1));
            beens.insert((y, x-1, false));
        }
        if y < height-1 && board[y+1][x] == '1' && !beens.contains(&(y+1, x, false)) {
            queue.push_back((y+1, x, false, distance+1));
            beens.insert((y+1, x, false));
        }
        if x < width-1 && board[y][x+1] == '1' && !beens.contains(&(y, x+1, false)) {
            queue.push_back((y, x+1, false, distance+1));
            beens.insert((y, x+1, false));
        }
    }

    println!("-1");
}
