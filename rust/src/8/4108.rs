use std::io::stdin;

fn tick() -> bool {
    let (height, width) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());

        (iter.next().unwrap(), iter.next().unwrap())
    };

    if height == 0 && width == 0 {
        return true;
    }

    let board = {
        let mut result = Vec::new();

        for _ in 0..height {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let mut row = Vec::new();
            for c in input.trim().chars() {
                row.push(c);
            }
            result.push(row);
        }

        result
    };

    for i in 0..height {
        for j in 0..width {
            if board[i][j] == '*' {
                print!("*");
                continue;
            }

            let mut mines = 0;

            // LU
            if i > 0 && j > 0 && board[i-1][j-1] == '*' {
                mines += 1;
            }
            // L
            if j > 0 && board[i][j-1] == '*' {
                mines += 1;
            }
            // LD
            if i < height-1 && j > 0 && board[i+1][j-1] == '*' {
                mines += 1;
            }
            // D
            if i < height-1 && board[i+1][j] == '*' {
                mines += 1;
            }
            // RD
            if i < height-1 && j < width-1 && board[i+1][j+1] == '*' {
                mines += 1;
            }
            // R
            if j < width-1 && board[i][j+1] == '*' {
                mines += 1;
            }
            // RU
            if j < width-1 && i > 0 && board[i-1][j+1] == '*' {
                mines += 1;
            }
            // U
            if i > 0 && board[i-1][j] == '*' {
                mines += 1;
            }

            print!("{}", mines);
        }
        println!();
    }

    return false;
}

fn main() {
    while !tick() { }
}
