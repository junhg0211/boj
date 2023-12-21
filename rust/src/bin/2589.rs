use std::io::stdin;
use std::collections::{ HashSet, VecDeque };

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
        let row = input
            .trim()
            .chars()
            .collect::<Vec<_>>();
        board.push(row);
    }

    let mut result = 0;
    let mut beens = HashSet::new();
    for i in 0..height {
        for j in 0..width {
            if board[i][j] != 'L' {
                continue;
            }

            if beens.contains(&(i, j)) {
                continue;
            }

            // println!("{} {}", i, j);

            let mut end = (i, j, 0);

            // -- find ends
            let mut queue = VecDeque::new();
            queue.push_back((i, j, 0));
            beens.insert((i, j));
            while let Some(thing) = queue.pop_front() {
                let (y, x, distance) = thing;

                if distance > end.2 {
                    end = thing;
                }

                if y > 0 && board[y-1][x] == 'L' && !beens.contains(&(y-1, x)) {
                    queue.push_back((y-1, x, distance+1));
                    beens.insert((y-1, x));
                }

                if x > 0 && board[y][x-1] == 'L' && !beens.contains(&(y, x-1)) {
                    queue.push_back((y, x-1, distance+1));
                    beens.insert((y, x-1));
                }

                if y < height-1 && board[y+1][x] == 'L' && !beens.contains(&(y+1, x)) {
                    queue.push_back((y+1, x, distance+1));
                    beens.insert((y+1, x));
                }

                if x < width-1 && board[y][x+1] == 'L' && !beens.contains(&(y, x+1)) {
                    queue.push_back((y, x+1, distance+1));
                    beens.insert((y, x+1));
                }
            }

            // -- find distance
            queue.clear();
            let mut been = HashSet::new();
            queue.push_back((end.0, end.1, 0));
            been.insert((end.0, end.1));
            while let Some(thing) = queue.pop_front() {
                let (y, x, distance) = thing;

                if distance > result {
                    result = distance;
                }

                if y > 0 && board[y-1][x] == 'L' && !been.contains(&(y-1, x)) {
                    queue.push_back((y-1, x, distance+1));
                    been.insert((y-1, x));
                }

                if x > 0 && board[y][x-1] == 'L' && !been.contains(&(y, x-1)) {
                    queue.push_back((y, x-1, distance+1));
                    been.insert((y, x-1));
                }

                if y < height-1 && board[y+1][x] == 'L' && !been.contains(&(y+1, x)) {
                    queue.push_back((y+1, x, distance+1));
                    been.insert((y+1, x));
                }

                if x < width-1 && board[y][x+1] == 'L' && !been.contains(&(y, x+1)) {
                    queue.push_back((y, x+1, distance+1));
                    been.insert((y, x+1));
                }
            };
        }
    }

    println!("{}", result);
}
