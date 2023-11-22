use std::io::stdin;
use std::collections::{ HashSet, VecDeque };

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let height = iter.next().unwrap();
    let width = iter.next().unwrap();

    let mut board = Vec::new();
    for _ in 0..height {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut row = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        board.push(row);
    }

    let mut bongs = HashSet::new();
    let mut result = 0;
    for i in 0..height {
        for j in 0..width {
            if bongs.contains(&(i, j)) {
                continue;
            }

            let mut queue = VecDeque::new();
            let mut beens = HashSet::new();
            queue.push_back((i, j));
            beens.insert((i, j));

            let mut is_bong = true;
            while let Some((y, x)) = queue.pop_front() {
                // println!("{} {} {} {} {:?} {:?}", i, j, y, x, queue, beens);
                let this = board[y][x];

                // check bong vailidity
                if y > 0 && board[y-1][x] > this
                        || x > 0 && board[y][x-1] > this
                        || y < height-1 && board[y+1][x] > this
                        || x < width-1 && board[y][x+1] > this
                        || y > 0 && x > 0 && board[y-1][x-1] > this
                        || y > 0 && x < width-1 && board[y-1][x+1] > this
                        || y < height-1 && x > 0 && board[y+1][x-1] > this
                        || y < height-1 && x < width-1 && board[y+1][x+1] > this
                {
                    is_bong = false;
                    break;
                }

                // check neighbor bong
                if y > 0 && board[y-1][x] == this && !beens.contains(&(y-1, x)) {
                    queue.push_back((y-1, x));
                    beens.insert((y-1, x));
                }
                if x > 0 && board[y][x-1] == this && !beens.contains(&(y, x-1)) {
                    queue.push_back((y, x-1));
                    beens.insert((y, x-1));
                }
                if y < height-1 && board[y+1][x] == this && !beens.contains(&(y+1, x)) {
                    queue.push_back((y+1, x));
                    beens.insert((y+1, x));
                }
                if x < width-1 && board[y][x+1] == this && !beens.contains(&(y, x+1)) {
                    queue.push_back((y, x+1));
                    beens.insert((y, x+1));
                }
                if y > 0 && x > 0 && board[y-1][x-1] == this && !beens.contains(&(y-1, x-1)) {
                    queue.push_back((y-1, x-1));
                    beens.insert((y-1, x-1));
                }
                if y > 0 && x < width-1 && board[y-1][x+1] == this && !beens.contains(&(y-1, x+1)) {
                    queue.push_back((y-1, x+1));
                    beens.insert((y-1, x+1));
                }
                if y < height-1 && x > 0 && board[y+1][x-1] == this && !beens.contains(&(y+1, x-1)) {
                    queue.push_back((y+1, x-1));
                    beens.insert((y+1, x-1));
                }
                if y < height-1 && x < width-1 && board[y+1][x+1] == this && !beens.contains(&(y+1, x+1)) {
                    queue.push_back((y+1, x+1));
                    beens.insert((y+1, x+1));
                }
            }

            if is_bong {
                result += 1;
                for been in beens.drain() {
                    bongs.insert(been);
                }
            }
        }
    }

    // println!("{:?}", bongs);
    println!("{}", result);
}
