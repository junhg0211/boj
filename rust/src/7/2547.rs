use std::io::stdin;

fn tick() {
    let mut input = String::new();

    // get one line and throw it away
    stdin().read_line(&mut input).unwrap();
    input.clear();

    // get count
    let count = {
        stdin().read_line(&mut input).unwrap();
        input.trim().parse::<u64>().unwrap()
    };

    // calculate sum
    let mut sum = 0;
    for _ in 0..count {
        input.clear();

        stdin().read_line(&mut input).unwrap();
        sum += input.trim().parse::<u64>().unwrap();
        sum %= count;
    }

    // print result
    if sum % count == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn main() {
    // get testcase count
    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u64>().unwrap()
    };

    // tick testcases
    for _ in 0..count {
        tick();
    }
}
