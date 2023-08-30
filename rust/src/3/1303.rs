use std::io::stdin;

fn main() {
    let (width, height) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());

        (iter.next().unwrap(), iter.next().unwrap())
    };

    let mut board = {
        let mut result = Vec::new();

        for _ in 0..height {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            result.push(input.trim().chars().collect::<Vec<_>>());
        }

        result
    };

    let mut white = 0;
    let mut blue = 0;

    for i in 0..height {
        for j in 0..width {
            if board[i][j] == ' ' {
                continue;
            }

            // println!("{:?} {}", board, board[i][j]);

            let mut dfs = vec![(i, j)];
            let mut count = 0;
            let color = board[i][j];

            while !dfs.is_empty() {
                let (i, j) = dfs.pop().unwrap();

                if board[i][j] == ' ' {
                    continue;
                }

                board[i][j] = ' ';
                count += 1;

                // print!("({}, {}) ", i, j);

                let deltas = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
                for (dy, dx) in deltas {
                    let (y, x) = (i as i32 + dy, j as i32 + dx);

                    if x < 0 || x > width as i32 - 1 || y < 0 || y > height as i32 - 1 {
                        continue;
                    }

                    let (x, y) = (x as usize, y as usize);

                    if board[y][x] != color {
                        continue;
                    }

                    dfs.push((y, x));
                }
            }

            // println!("{} {} {} {}", i, j, color, count);
            if color == 'W' {
                white += count * count;
            } else {
                blue += count * count;
            }
        }
    }

    println!("{} {}", white, blue);
}
