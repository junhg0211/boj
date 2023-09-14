use std::io::{ stdin, stdout, BufWriter, Write };
use std::collections::VecDeque;

fn main() {
    // --- get board
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
    let mut target = (0, 0);
    for i in 0..height {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut row = Vec::new();
        for (j, value) in input
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .enumerate() {
            row.push(value);

            if value == 2 {
                target = (i, j);
            }
        }

        board.push(row);
    }

    // --- bfs
    let mut been = vec![vec![false; width]; height];
    let mut queue = VecDeque::new();
    queue.push_back(target);
    been[target.0][target.1] = true;
    board[target.0][target.1] = 0;

    while !queue.is_empty() {
        let (y, x) = queue.pop_front().unwrap();

        // up
        if y > 0 && !been[y-1][x] && board[y-1][x] != 0 {
            board[y-1][x] = board[y][x] + 1;
            been[y-1][x] = true;
            queue.push_back((y-1, x));
        }

        // left
        if x > 0 && !been[y][x-1] && board[y][x-1] != 0 {
            board[y][x-1] = board[y][x] + 1;
            been[y][x-1] = true;
            queue.push_back((y, x-1));
        }

        // down
        if y < height-1 && !been[y+1][x] && board[y+1][x] != 0 {
            board[y+1][x] = board[y][x] + 1;
            been[y+1][x] = true;
            queue.push_back((y+1, x));
        }

        // right
        if x < width-1 && !been[y][x+1] && board[y][x+1] != 0 {
            board[y][x+1] = board[y][x] + 1;
            been[y][x+1] = true;
            queue.push_back((y, x+1));
        }
    }

    // --- handle not beens
    for i in 0..height {
        for j in 0..width {
            if been[i][j] {
                continue;
            }

            if board[i][j] == 0 {
                continue;
            }

            board[i][j] = -1;
        }
    }

    // --- print result
    let mut writer = BufWriter::new(stdout());
    for row in board {
        for cell in row {
            write!(writer, "{} ", cell).unwrap();
        }
        writeln!(writer).unwrap();
    }
}
