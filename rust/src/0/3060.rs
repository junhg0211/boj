use std::io::stdin;

fn tick() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut saryo = input.trim().parse::<u32>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut needs = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut day = 1;
    loop {
        let mut need = needs.iter().sum::<u32>();

        if need > saryo {
            println!("{}", day);
            return;
        }

        needs = vec![
            needs[0] + needs[1] + needs[3] + needs[5],
            needs[1] + needs[0] + needs[2] + needs[4],
            needs[2] + needs[1] + needs[3] + needs[5],
            needs[3] + needs[0] + needs[2] + needs[4],
            needs[4] + needs[1] + needs[3] + needs[5],
            needs[5] + needs[0] + needs[2] + needs[4],
        ];

        day += 1;
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let testcase_count = input.trim().parse::<usize>().unwrap();

    for _ in 0..testcase_count {
        tick();
    }
}
