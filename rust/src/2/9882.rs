use std::io::stdin;

fn f(scores: &Vec<u32>, teams: &mut Vec<usize>, mut availables: [u32; 4]) -> u32 {
    // get possilbes
    let mut possibles = Vec::new();
    for i in 0..4 {
        if availables[i] > 0 {
            possibles.push(i);
        }
    }

    if possibles.is_empty() {
        let mut powers = vec![0, 0, 0, 0];
        for i in 0..12 {
            powers[teams[i]] += scores[i];
        }

        return powers.iter().max().unwrap() - powers.iter().min().unwrap();
    }

    let mut record = u32::MAX;
    for &possible in possibles.iter() {
        teams.push(possible);
        availables[possible] -= 1;
        record = u32::min(record, f(scores, teams, availables));
        availables[possible] += 1;
        teams.pop();
    }

    return record;
}

fn main() {
    let mut numbers = Vec::new();

    for _ in 0..12 {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        numbers.push(input.trim().parse::<u32>().unwrap());
    }

    println!("{}", f(&numbers, &mut Vec::new(), [3, 3, 3, 3]));
}
