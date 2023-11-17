use std::io::stdin;
use std::collections::{ HashMap, HashSet };

fn main() {
    let mut board = Vec::new();
    for _ in 0..9 {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let row = input
            .trim()
            .chars()
            .map(|x| x as usize - '0' as usize)
            .collect::<Vec<_>>();
        board.push(row);
    }

    let mut noses: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    let mut i = 0;
    while i < 9 {
        let mut j = 0;
        while j < 9 {
            if board[i][j] != 0 {
                j += 1;
                continue;
            }

            // possibles
            let mut possibles = vec![true; 9];
            for k in 0..9 {
                if board[i][k] != 0 {
                    possibles[board[i][k]-1] = false;
                }
                if board[k][j] != 0 {
                    possibles[board[k][j]-1] = false;
                }

                let y = i/3*3 + k/3;
                let x = j/3*3 + k%3;
                if board[y][x] != 0 {
                    possibles[board[y][x]-1] = false;
                }
            }
            let empty_vector = Vec::new();
            let nos = noses.get(&(i, j)).unwrap_or(&empty_vector);
            let mut possibles = possibles
                .iter()
                .enumerate()
                .filter(|x| *x.1 && !nos.contains(&(x.0+1)))
                .map(|x| x.0 + 1)
                .collect::<Vec<_>>();
            possibles.sort();

            println!("{:?}", possibles);

            j += 1;
        }

        i += 1;
    }

    println!("{:?}", board);
}
