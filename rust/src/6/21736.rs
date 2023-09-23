use std::io::stdin;
use std::collections::{ VecDeque, HashSet };

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
    let (mut x, mut y) = (0, 0);
    for i in 0..height {
        let row = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            input.trim().chars().collect::<Vec<char>>()
        };

        board.push(row);

        for j in 0..width {
            if board[i][j] == 'I' {
                y = i;
                x = j;
            }
        }
    }

    let mut friends = 0;
    let mut queue = VecDeque::new();
    let mut been = HashSet::new();
    queue.push_back((x, y));
    been.insert((x, y));

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();

        if board[y][x] == 'P' {
            friends += 1;
        }

        if x > 0 && !been.contains(&(x-1, y)) && board[y][x-1] != 'X' {
            queue.push_back((x-1, y));
            been.insert((x-1, y));
        }
        if y > 0 && !been.contains(&(x, y-1)) && board[y-1][x] != 'X' {
            queue.push_back((x, y-1));
            been.insert((x, y-1));
        }
        if x < width-1 && !been.contains(&(x+1, y)) && board[y][x+1] != 'X' {
            queue.push_back((x+1, y));
            been.insert((x+1, y));
        }
        if y < height-1 && !been.contains(&(x, y+1)) && board[y+1][x] != 'X' {
            queue.push_back((x, y+1));
            been.insert((x, y+1));
        }
    }

    match friends {
        0 => println!("TT"),
        friends => println!("{}", friends),
    }
}
