use std::io::stdin;
use std::collections::HashMap;

fn get_ways(x: usize, y: usize, dp: &mut HashMap<(usize, usize), u32>, board: &Vec<Vec<usize>>, size: usize) -> u32 {
    if dp.contains_key(&(x, y)) {
        return *dp.get(&(x, y)).unwrap();
    }
    if y == x && x == size-1 {
        return 1;
    }

    let distance = board[y][x];

    let down = if y + distance < size { get_ways(x, y+distance, dp, board, size) } else { 0 };
    let right = if x + distance < size { get_ways(x+distance, y, dp, board, size) } else { 0 };

    dp.insert((x, y), down + right);
    return down + right;
}

fn main() {
    // get size
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let size = input.trim().parse::<usize>().unwrap();

    // get board
    let mut board = Vec::new();
    for _ in 0..size {
        input.clear();
        stdin().read_line(&mut input).unwrap();

        board.push(input.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>());
    }

    // print count
    let mut dp = HashMap::new();
    println!("{}", get_ways(0, 0, &mut dp, &board, size));
}
