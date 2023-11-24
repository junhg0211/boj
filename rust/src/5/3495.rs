use std::io::stdin;
use std::collections::{ HashSet, VecDeque };

fn main() {
    // -- get board size
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let height = iter.next().unwrap();
    let width = iter.next().unwrap();

    // -- get board
    let mut board = Vec::new();
    let mut half_count = 0;
    for _ in 0..height {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let row = input.trim().chars().collect::<Vec<_>>();

        for &letter in row.iter() {
            if letter == '.' {
                continue;
            }

            half_count += 1;
        }

        board.push(row);
    }

    // -- get inside area
    let mut inside_area = 0;
    let mut beens = HashSet::new();
    let mut queue = VecDeque::new();
    for i in 0..height {
        for j in 0..width {
            if board[i][j] != '.' {
                continue;
            }

            if beens.contains(&(i, j)) {
                continue;
            }

            let mut is_outside = false;
            queue.push_back((i, j));
            beens.insert((i, j));
            let mut count = 0;

            while let Some((i, j)) = queue.pop_front() {
                count += 1;

                if i == 0 || i == height-1 || j == 0 || j == width-1 {
                    is_outside = true;
                }

                if i > 0 && board[i-1][j] == '.' && !beens.contains(&(i-1, j)) {
                    queue.push_back((i-1, j));
                    beens.insert((i-1, j));
                }
                if j > 0 && board[i][j-1] == '.' && !beens.contains(&(i, j-1)) {
                    queue.push_back((i, j-1));
                    beens.insert((i, j-1));
                }
                if i < height-1 && board[i+1][j] == '.' && !beens.contains(&(i+1, j)) {
                    queue.push_back((i+1, j));
                    beens.insert((i+1, j));
                }
                if j < width-1 && board[i][j+1] == '.' && !beens.contains(&(i, j+1)) {
                    queue.push_back((i, j+1));
                    beens.insert((i, j+1));
                }
            }

            if !is_outside {
                inside_area += count;
            }
        }
    }

    // -- calculate result and print
    let result = inside_area + half_count / 2;
    println!("{}", result);
}
