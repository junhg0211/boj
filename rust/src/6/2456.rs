use std::io::stdin;

fn main() {
    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    let mut scores = [0; 3];
    let mut threes = [0; 3];
    let mut twos = [0; 3];

    for _ in 0..count {
        let votes = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            input
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        };

        for i in 0..3 {
            scores[i] += votes[i];

            if votes[i] == 3 {
                threes[i] += 1;
            } else if votes[i] == 2 {
                twos[i] += 1;
            }
        }
    }

    let mut wow = (0..3).map(|i| (scores[i], threes[i], twos[i], i + 1)).collect::<Vec<_>>();
    wow.sort();

    // println!("{:?}", wow);

    if wow[2].0 == wow[1].0
            && wow[2].1 == wow[1].1
            && wow[2].2 == wow[1].2 {
        scores.sort();
        println!("0 {}", scores[2]);
    } else {
        println!("{} {}", wow[2].3, wow[2].0);
    }
}
