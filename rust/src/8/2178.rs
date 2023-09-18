use std::io::stdin;
use std::collections::VecDeque;

fn main() {
    let (height, width) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());

        (
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    };

    let mut board = Vec::new();
    for _ in 0..height {
        let row = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            input
                .trim()
                .chars()
                .collect::<Vec<_>>()
        };

        board.push(row);
    }

    let mut been = vec![vec![false; width]; height];

    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 1));
    loop {
        let (x, y, distance) = queue.pop_front().unwrap();

        if x == width-1 && y == height-1 {
            println!("{}", distance);
            return;
        }

        if y > 0 && !been[y-1][x] && board[y-1][x] == '1' {
            queue.push_back((x, y-1, distance+1));
            been[y-1][x] = true;
        }

        if x > 0 && !been[y][x-1] && board[y][x-1] == '1' {
            queue.push_back((x-1, y, distance+1));
            been[y][x-1] = true;
        }

        if y < height-1 && !been[y+1][x] && board[y+1][x] == '1' {
            queue.push_back((x, y+1, distance+1));
            been[y+1][x] = true;
        }

        if x < width-1 && !been[y][x+1] && board[y][x+1] == '1' {
            queue.push_back((x+1, y, distance+1));
            been[y][x+1] = true;
        }
    }
}
