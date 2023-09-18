use std::io::{ stdin, stdout, BufWriter, Write };

fn main() {
    let size = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<usize>().unwrap()
    };

    let mut board = Vec::new();
    for _ in 0..size {
        let row = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            input.trim().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>()
        };

        board.push(row);
    }

    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
                if board[i][j] == 1 && board[k][i] == 1 {
                    board[k][j] = 1;
                }
            }
        }
    }

    let mut writer = BufWriter::new(stdout());
    for i in 0..size {
        for j in 0..size {
            write!(writer, "{} ", board[i][j]).unwrap();
        }
        writeln!(writer).unwrap()
    }
}
