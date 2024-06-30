use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let subject_count = iter.next().unwrap();
    let time_available = iter.next().unwrap();

    let mut subjects = Vec::new();
    for _ in 0..subject_count {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());
        let time = iter.next().unwrap();
        let score = iter.next().unwrap();

        subjects.push((time, score));
    }

    let mut scores = vec![vec![0; time_available + 1]; subject_count + 1];
    for i in 1..=subject_count {
        let (time, score) = subjects[i - 1];
        for j in 1..=time_available {
            let a = scores[i - 1][j];
            let b = if j < time {
                0
            } else {
                scores[i - 1][j - time] + score
            };

            scores[i][j] = usize::max(a, b);
        }
    }

    // println!("{:?}", scores);
    println!("{}", scores[subject_count][time_available]);
}
