use std::io::{ stdin, stdout, Write, BufWriter };

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let number_count = input.trim().parse::<usize>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut dp = vec![vec![false; number_count]; number_count];
    for length in 0..number_count {
        for i in 0..number_count-length {
            if length == 0 {
                dp[i][i] = true;
                continue;
            }

            if length == 1 {
                dp[i][i+1] = numbers[i] == numbers[i+1];
                continue;
            }

            let j = i+length;
            dp[i][j] =  numbers[i] == numbers[j] && dp[i+1][j-1];
        }
    }

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let question_count = input.trim().parse::<usize>().unwrap();

    let mut writer = BufWriter::new(stdout());
    for _ in 0..question_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());

        let start = iter.next().unwrap() - 1;
        let end = iter.next().unwrap() - 1;

        let answer = if dp[start][end] {
            "1"
        } else {
            "0"
        };
        writeln!(writer, "{}", answer).unwrap();
    }
}

