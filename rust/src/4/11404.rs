use std::io::{ stdin, stdout, Write, BufWriter };

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let size = input.trim().parse::<usize>().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let testcase_count = input.trim().parse::<usize>().unwrap();

    let mut board = vec![vec![usize::MAX; size]; size];
    let mut connected = vec![vec![false; size]; size];
    for i in 0..size {
        board[i][i] = 0;
        connected[i][i] = true;
    }

    for _ in 0..testcase_count {
        input.clear();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());

        let y = iter.next().unwrap() - 1;
        let x = iter.next().unwrap() - 1;
        let distance = iter.next().unwrap();

        if distance < board[y][x] {
            board[y][x] = distance;
            connected[y][x] = true;
        }
    }

    for i in 0..size {
        for j in 0..size {
            if !connected[i][j] {
                continue;
            }

            for k in 0..size {
                if !connected[k][i] {
                    continue;
                }

                let new_distance = board[i][j] + board[k][i];
                if new_distance < board[k][j] {
                    board[k][j] = new_distance;
                    connected[k][j] = true;
                }
            }
        }
    }

    let mut writer = BufWriter::new(stdout());
    for i in 0..size {
        for j in 0..size {
            if i == j {
                write!(writer, "0 ").unwrap();
                continue;
            }
            if !connected[i][j] {
                write!(writer, "0 ").unwrap();
                continue;
            }

            write!(writer, "{} ", board[i][j]).unwrap();
        }
        writeln!(writer).unwrap();
    }
}
