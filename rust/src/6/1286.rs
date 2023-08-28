use std::io::stdin;

fn main() {
    // --- get board
    let (height, width) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());

        (iter.next().unwrap(), iter.next().unwrap())
    };

    let mut board = Vec::new();

    for i in 0..height {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        board.push(input.trim().to_owned());
    }

    // --- count
    // counter
    let mut counter = Vec::new();

    for c in 'A'..='Z' {
        let mut new_counter = (0..height * 2)
            .map(|_| (0..width * 2)
                 .map(|_| 0)
                 .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        counter.push(new_counter);

        for i in 0..(height*2) as usize {
            for j in 0..(width*2) as usize {
                let ci = c as usize - 'A' as usize; // char index

                let up = if i == 0 { 0 } else { counter[ci][i-1][j] };
                let left = if j == 0 { 0 } else { counter[ci][i][j-1] };
                let diagonal = if i == 0 || j == 0 { 0 } else { counter[ci][i-1][j-1] };
                let is = (board[i % height as usize].chars().nth(j % width as usize).unwrap() == c) as u32;

                counter[ci][i][j] = up + left - diagonal + is;
            }
        }
    }

    // count
    for c in 0..26 {
        let mut count = 0;

        for w in 1..=(width*2) as usize {
            for h in 1..=(height*2) as usize {
                for x in 0..=((width*2) as usize - w) {
                    for y in 0..=((height*2) as usize - h) {
                        let yellow = counter[c][y+h-1][x+w-1];
                        let blue = if y == 0 { 0 } else { counter[c][y-1][x+w-1] };
                        let pink = if x == 0 { 0 } else { counter[c][y+h-1][x-1] };
                        let green = if x == 0 || y == 0 { 0 } else { counter[c][y-1][x-1] };

                        count += yellow + green - blue - pink;
                    }
                }
            }
        }

        println!("{}", count);
    }
}
