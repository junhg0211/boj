use std::io::stdin;

fn input() -> i32 {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let mut a = input();
    let b = input();
    let c = input();
    let d = input();
    let e = input();

    let mut frozen = a < 0;
    let mut time = 0;
    while a < b {
        if frozen && a < 0 {
            time += c;
            a += 1;
        } else if frozen && a == 0 {
            time += d;
            frozen = false;
        } else {
            time += e;
            a += 1;
        }
    }

    println!("{}", time);
}
