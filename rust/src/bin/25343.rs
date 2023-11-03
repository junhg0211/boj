use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let size = input.trim().parse::<usize>().unwrap();

    let mut board = Vec::new();
    for _ in 0..size {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let row = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        board.push(row);
    }

    let mut dp = vec![vec![(0, 0); size]; size];
    // (length, max)
    for i in 0..size {
        for j in 0..size {
            let mut will = (1, board[i][j]);

            if i > 0 {
                let (length, max) = dp[i-1][j];
                if board[i][j] > max {
                    will = (length+1, board[i][j]);
                }
                if length > will.0 {
                    will = (length, max);
                }
            }

            if j > 0 {
                let (length, max) = dp[i][j-1];
                if board[i][j] > max {
                    will = (length+1, board[i][j]);
                }
                if length > will.0 {
                    will = (length, max);
                }
            }

            dp[i][j] = will;
        }
    }

    for i in 0..size {
        println!("{:?}", dp[i]);
    }

    println!("{}", dp[size-1][size-1].0);
}
