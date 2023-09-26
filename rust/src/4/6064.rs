use std::io::stdin;

fn gcd(mut a: u32, mut b: u32) -> u32 {
    if a < b {
        (a, b) = (b, a);
    }

    while b > 0 {
        (a, b) = (b, a%b);
    }

    a
}

fn tick() {
    // get m, n, x, y
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    let m = iter.next().unwrap();
    let n = iter.next().unwrap();
    let mut x = iter.next().unwrap();
    let mut y = iter.next().unwrap();

    // check validality
    let g = gcd(m, n);
    if x > y && (x-y) % g != 0
            || x < y && (y-x) % g != 0 {
        println!("-1");
        return;
    }

    // calculate year
    while x != y {
        if x < y {
            x += m;
        } else {
            y += n;
        }
    }

    // print result
    println!("{}", x);
}

fn main() {
    // get testcase count
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let testcase_count = input.trim().parse::<u32>().unwrap();

    // repeat `count` times
    for _ in 0..testcase_count {
        tick();
    }
}
