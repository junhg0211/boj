use std::io::stdin;
use std::collections::{ VecDeque, HashSet };

fn main() {
    // -- get data from stdin
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let size = input.trim().parse::<usize>().unwrap();

    let mut board = Vec::new();
    let (mut now_x, mut now_y) = (0, 0);
    for i in 0..size {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut row = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        for j in 0..size {
            if row[j] == 9 {
                now_y = i;
                now_x = j;
                row[j] = 0;
                break;
            }
        }

        board.push(row);
    }

    // -- simulate baby shart
    let mut level = 2;
    let mut exp = 0;
    let mut result = 0;
    loop {
        // find closest prey with bfs and move
        let mut queue = VecDeque::new();
        let mut beens = HashSet::new();
        queue.push_back((now_y, now_x, 0));
        beens.insert((now_y, now_x));
        let mut not_found = true;
        let (mut food_y, mut food_x, mut food_distance) = (0, 0, 0);
        loop {
            let (y, x, distance) = match queue.pop_front() {
                Some(thing) => thing,
                None => break,
            };

            if board[y][x] < level && board[y][x] != 0 {
                if !not_found && distance > food_distance {
                    break;
                }

                if not_found || y < food_y || y == food_y && x < food_x {
                    food_y = y;
                    food_x = x;
                    food_distance = distance;
                }

                not_found = false;
            }

            if y > 0 && !beens.contains(&(y-1, x)) && board[y-1][x] <= level {
                queue.push_back((y-1, x, distance+1));
                beens.insert((y-1, x));
            }
            if x > 0 && !beens.contains(&(y, x-1)) && board[y][x-1] <= level {
                queue.push_back((y, x-1, distance+1));
                beens.insert((y, x-1));
            }
            if y < size-1 && !beens.contains(&(y+1, x)) && board[y+1][x] <= level {
                queue.push_back((y+1, x, distance+1));
                beens.insert((y+1, x));
            }
            if x < size-1 && !beens.contains(&(y, x+1)) && board[y][x+1] <= level {
                queue.push_back((y, x+1, distance+1));
                beens.insert((y, x+1));
            }
        }

        if not_found {
            break;
        }

        // process eating
        result += food_distance;
        now_y = food_y;
        now_x = food_x;
        exp += 1;
        board[food_y][food_x] = 0;

        if exp >= level {
            level += 1;
            exp = 0;
        }

        /*
        // print board
        for i in 0..size {
            println!("{:?}", board[i]);
        }
        println!("result={}, level={}, exp={}", result, level, exp);
        println!("x={}, y={}", now_y, now_x);
        println!();
        */
    }

    println!("{}", result);
}
