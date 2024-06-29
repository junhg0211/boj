use std::io::stdin;

fn tick() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<usize>().unwrap();

    let mut in_outs = Vec::new();
    for _ in 0..count {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());

        let mut a = iter.next().unwrap();
        let mut b = iter.next().unwrap();

        if a > b {
            (a, b) = (b, a);
        }

        in_outs.push(((a - 1) / 2, 0));
        in_outs.push(((b - 1) / 2, 1));
    }

    in_outs.sort();

    let mut result = 0;
    let mut sum = 0;
    for &(_, in_out) in in_outs.iter() {
        if in_out == 0 {
            sum += 10;
        } else {
            sum -= 10;
        }

        result = u32::max(result, sum);
    }

    println!("{}", result);
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let testcase_count = input.trim().parse::<usize>().unwrap();

    for _ in 0..testcase_count {
        tick();
    }
}
