use std::io::stdin;
use std::collections::{ HashSet, VecDeque, HashMap };

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let height = iter.next().unwrap()+2;
    let width = iter.next().unwrap()+2;

    let mut board = Vec::new();
    board.push(vec![0; width]);
    for _ in 0..height-2 {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut row = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        row.insert(0, 0);
        row.push(0);

        board.push(row);
    }
    board.push(vec![0; width]);

    let mut result = 0;
    loop {
        let mut queue = VecDeque::new();
        let mut beens = HashSet::new();
        queue.push_back((0, 0));
        beens.insert((0, 0));
        let mut neighbors = HashMap::new();

        loop {
            let (y, x) = if let Some(thing) = queue.pop_front() {
                thing
            } else {
                break
            };

            if y > 0 {
                if board[y-1][x] == 1 {
                    neighbors.insert(
                        (y-1, x),
                        *neighbors.get(&(y-1, x)).unwrap_or(&0) + 1
                    );
                } else if !beens.contains(&(y-1, x)) {
                    queue.push_back((y-1, x));
                    beens.insert((y-1, x));
                }
            }
            if x > 0 {
                if board[y][x-1] == 1 {
                    neighbors.insert(
                        (y, x-1),
                        *neighbors.get(&(y, x-1)).unwrap_or(&0) + 1
                    );
                } else if !beens.contains(&(y, x-1)) {
                    queue.push_back((y, x-1));
                    beens.insert((y, x-1));
                }
            }
            if y < height-1 {
                if board[y+1][x] == 1 {
                    neighbors.insert(
                        (y+1, x),
                        *neighbors.get(&(y+1, x)).unwrap_or(&0) + 1
                    );
                } else if !beens.contains(&(y+1, x)) {
                    queue.push_back((y+1, x));
                    beens.insert((y+1, x));
                }
            }
            if x < width-1 {
                if board[y][x+1] == 1 {
                    neighbors.insert(
                        (y, x+1),
                        *neighbors.get(&(y, x+1)).unwrap_or(&0) + 1
                    );
                } else if !beens.contains(&(y, x+1)) {
                    queue.push_back((y, x+1));
                    beens.insert((y, x+1));
                }
            }
        }

        if neighbors.len() == 0 {
            break;
        }

        for (&(y, x), &neighbor_count) in &neighbors {
            if neighbor_count >= 2 {
                board[y][x] = 0;
            }
        }

        neighbors.clear();
        beens.drain();
        result += 1;

        /*
        println!("{}", result);
        for row in &board {
            println!("{:?}", row);
        }
        println!();
        */
    }

    println!("{}", result)
}
