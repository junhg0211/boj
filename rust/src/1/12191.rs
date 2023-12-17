use std::io::stdin;
use std::collections::VecDeque;

fn tick(case_index: usize) {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let size = input.trim().parse::<usize>().unwrap();

    let mut board = Vec::new();
    for _ in 0..size {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let row = input
            .trim()
            .chars()
            .collect::<Vec<_>>();

        board.push(row);
    }

    let mut neighbors = Vec::new();
    for i in 0..size {
        let mut row = Vec::new();

        for j in 0..size {
            if board[i][j] == '*' {
                row.push(9);
                continue;
            }

            let mut neighbor = 0;

            if i > 0 && board[i-1][j] == '*' { neighbor += 1; }
            if i < size-1 && board[i+1][j] == '*' { neighbor += 1; }
            if j > 0 && board[i][j-1] == '*' { neighbor += 1; }
            if j < size-1 && board[i][j+1] == '*' { neighbor += 1; }

            if i > 0 && j > 0 && board[i-1][j-1] == '*' { neighbor += 1; }
            if i > 0 && j < size-1 && board[i-1][j+1] == '*' { neighbor += 1; }
            if i < size-1 && j > 0 && board[i+1][j-1] == '*' { neighbor += 1; }
            if i < size-1 && j < size-1 && board[i+1][j+1] == '*' { neighbor += 1; }

            row.push(neighbor);
        }

        neighbors.push(row);
    }

    let mut beens = vec![vec![false; size]; size];
    let mut clicks = 0;

    for i in 0..size {
        for j in 0..size {
            if neighbors[i][j] != 0 {
                continue;
            }
            if beens[i][j] {
                continue;
            }

            let mut stack = VecDeque::new();
            stack.push_back((i, j));
            beens[i][j] = true;
            clicks += 1;

            while let Some(thing) = stack.pop_front() {
                let (y, x) = thing;

                if neighbors[y][x] != 0 {
                    continue;
                }

                if y > 0 && neighbors[y-1][x] != 9 && !beens[y-1][x] {
                    stack.push_back((y-1, x));
                    beens[y-1][x] = true;
                }

                if x > 0 && neighbors[y][x-1] != 9 && !beens[y][x-1] {
                    stack.push_back((y, x-1));
                    beens[y][x-1] = true;
                }

                if y < size-1 && neighbors[y+1][x] != 9 && !beens[y+1][x] {
                    stack.push_back((y+1, x));
                    beens[y+1][x] = true;
                }

                if x < size-1 && neighbors[y][x+1] != 9 && !beens[y][x+1] {
                    stack.push_back((y, x+1));
                    beens[y][x+1] = true;
                }

                if y > 0 && x > 0 && neighbors[y-1][x-1] != 9 && !beens[y-1][x-1] {
                    stack.push_back((y-1, x-1));
                    beens[y-1][x-1] = true;
                }

                if y > 0 && x < size-1 && neighbors[y-1][x+1] != 9 && !beens[y-1][x+1] {
                    stack.push_back((y-1, x+1));
                    beens[y-1][x+1] = true;
                }

                if y < size-1 && x > 0 && neighbors[y+1][x-1] != 9 && !beens[y+1][x-1] {
                    stack.push_back((y+1, x-1));
                    beens[y+1][x-1] = true;
                }

                if y < size-1 && x < size-1 && neighbors[y+1][x+1] != 9 && !beens[y+1][x+1] {
                    stack.push_back((y+1, x+1));
                    beens[y+1][x+1] = true;
                }
            }
        }
    }

    // println!("{}", clicks);

    for i in 0..size {
        for j in 0..size {
            if board[i][j] == '.' && !beens[i][j] {
                clicks += 1;
            }
        }
    }

    println!("Case #{}: {}", case_index+1, clicks);
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let testcase_count = input.trim().parse::<usize>().unwrap();

    for i in 0..testcase_count {
        tick(i);
    }
}
