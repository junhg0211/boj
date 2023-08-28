use std::io::stdin;

fn main() {
    // --- get board
    let (height, width) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap());

        (iter.next().unwrap(), iter.next().unwrap())
    };

    let mut board = Vec::new();

    for _ in 0..height {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        board.push(input.trim().to_owned());
    }

    // --- count
    for c in 0..26 {
        let mut count = 0;
        for i in 0..2 * height as usize {
            for j in 0..2 * width as usize {
                let w = width as usize;
                let h = height as usize;
                if board[i % h].chars().nth(j % w).unwrap() as u64 != 'A' as u64 + c {
                    continue;
                }

                let i = i as u64;
                let j = j as u64;

                count += (i+1)*(2*height-i) * (j+1)*(2*width-j);
            }
        }

        println!("{}", count);
    }
}
