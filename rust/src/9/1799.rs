use std::io::stdin;

fn next(number: usize, size: usize) -> usize {
    let (ni, nj) = (number / size, number % size);

    let mut result = number + 1;
    loop {
        let (i, j) = (result / size, result % size);

        if (ni + nj) % 2 == (i + j) % 2 {
            return result;
        }

        result += 1;
    }
}

fn check(possibles: &Vec<Vec<u32>>, shops: Vec<usize>, from: usize, size: usize) -> u32 {
    if from == size * size {
        return 0;
    }

    let mut result = 0;

    let fromi = from / size;
    let fromj = from % size;

    for position in from..size * size {
        let i = position / size;
        let j = position % size;

        if (fromi + fromj) % 2 != (i + j) % 2 {
            continue;
        }

        if possibles[i][j] == 0 {
            continue;
        }

        let mut good = true;
        for &shop in shops.iter() {
            let shopi = shop / size;
            let shopj = shop % size;

            if shopi + j == shopj + i {
                good = false;
                break;
            }

            if shopi + shopj == i + j {
                good = false;
                break;
            }
        }

        if !good {
            continue;
        }

        let mut new_shops = shops.clone();
        new_shops.push(position);

        let this_result = 1 + check(&possibles, new_shops, next(position, size), size);

        if this_result > result {
            result = this_result;
        }
    }

    return result;
}

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

    let result = check(&board, Vec::new(), 0, size) + check(&board, Vec::new(), 1, size);
    println!("{}", result);
}
