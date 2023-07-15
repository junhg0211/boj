use std::io::stdin;

fn get_matrix_size() -> (usize, usize) {
    // get matrix size
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    (numbers[0], numbers[1])
}

fn get_matrix() -> (Vec<Vec<i32>>, usize, usize) {
    // get size
    let (height, width) = get_matrix_size();

    let mut result = vec![vec![0; width]; height];

    for i in 0..height {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let numbers = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        result[i] = numbers;
    }

    (result, height, width)
}

fn main() {
    let (matrix1, r1, c1) = get_matrix();
    let (matrix2, _, c2) = get_matrix();

    // matrix multiplication
    let mut result = vec![vec![0; c2]; r1];

    for i in 0..r1 {
        for j in 0..c2 {
            for k in 0..c1 {
                result[i][j] += matrix1[i][k] * matrix2[k][j];
            }
        }
    }

    // print result
    for i in 0..r1 {
        for j in 0..c2 {
            print!("{} ", result[i][j]);
        }
        println!();
    }
}
