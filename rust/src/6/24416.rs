use std::io::stdin;

fn fib(n: usize) -> usize {
    if n == 1 || n == 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    println!("{} {}", fib(n), n - 2);
}
