use std::io::stdin;

fn tick() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap());

    let id = iter.next().unwrap();
    let stg = iter.next().unwrap();
    let mgm = iter.next().unwrap();
    let tch = iter.next().unwrap();
    let total = stg + mgm + tch;

    print!("{} {} ", id, total);

    if total < 55.0
    || stg/35.0 < 0.3
    || mgm/25.0 < 0.3
    || tch/40.0 < 0.3 {
        println!("FAIL");
    } else {
        println!("PASS");
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
