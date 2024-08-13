use std::io::stdin;

fn backtrack(s: &Vec<Vec<u32>>, teams: &mut Vec<bool>) -> u32 {
    if teams.len() == s.len() {
        let (mut start, mut link) = (0, 0);

        // start
        for i in 0..s.len() {
            if teams[i] {
                continue;
            }
            for j in 0..s.len() {
                if teams[j] {
                    continue;
                }
                start += s[i][j];
            }
        }

        // link
        for i in 0..s.len() {
            if !teams[i] {
                continue;
            }
            for j in 0..s.len() {
                if !teams[j] {
                    continue;
                }
                link += s[i][j];
            }
        }

        // println!("{} {}", start, link);
        return if start > link {
            start - link
        } else {
            link - start
        };
    }

    let trues = {
        let mut result = 0;
        for &team in teams.iter() {
            if team {
                result += 1;
            }
        }
        result
    };
    let falses = teams.len() - trues;

    let mut result = u32::MAX;

    if trues < s.len() / 2 {
        teams.push(true);

        result = u32::min(result, backtrack(s, teams));

        teams.pop().unwrap();
    }

    if falses < s.len() / 2 {
        teams.push(false);

        result = u32::min(result, backtrack(s, teams));

        teams.pop().unwrap();
    }

    result
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let player_count = input.trim().parse::<usize>().unwrap();

    let mut s = Vec::new();
    for _ in 0..player_count {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let row = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        s.push(row);
    }

    println!("{}", backtrack(&s, &mut Vec::new()));
}
