use std::io::stdin;

fn is_goable(board: &Vec<String>, y: usize, x: usize) -> bool {
    return board[y].chars().nth(x).unwrap() == '.';
}

fn get_index(y: usize, x: usize, width: usize) -> u32 {
    return (y * width + x) as u32;
}

fn is_been(been: u32, index: u32) -> bool {
    been & 2u32.pow(index) != 0
}

fn main() {
    // --- get board
    let (height, width, length) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());

        (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    };

    let board = {
        let mut result = Vec::new();

        for _ in 0..height {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            result.push(input.trim().to_owned());
        }

        result
    };

    // --- path finding
    let mut queue = Vec::new();

    queue.push((height-1, 0, 1, 2u32.pow(get_index(height-1, 0, width) as u32)));
               // pos_y, pos_x, length, been: u32

    let mut count = 0;

    while !queue.is_empty() {
        let (y, x, len, been) = queue.pop().unwrap();

        if y == 0 && x == width-1 && len == length {
            count += 1;
            // println!("{} {} {} {}", y, x, len, been);
            continue;
        }

        if len > length {
            continue;
        }

        // up
        if y > 0 && !is_been(been, get_index(y-1, x, width)) && is_goable(&board, y-1, x) {
            queue.push((y-1, x, len+1, been | 2u32.pow(get_index(y-1, x, width))));
        }
        // down
        if y < height-1 && !is_been(been, get_index(y+1, x, width)) && is_goable(&board, y+1, x) {
            queue.push((y+1, x, len+1, been | 2u32.pow(get_index(y+1, x, width))));
        }
        // left
        if x > 0 && !is_been(been, get_index(y, x-1, width)) && is_goable(&board, y, x-1) {
            queue.push((y, x-1, len+1, been | 2u32.pow(get_index(y, x-1, width))));
        }
        // right
        if x < width-1 && !is_been(been, get_index(y, x+1, width)) && is_goable(&board, y, x+1) {
            queue.push((y, x+1, len+1, been | 2u32.pow(get_index(y, x+1, width))));
        }
    }

    println!("{}", count);
}
