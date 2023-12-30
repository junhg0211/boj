use std::io::stdin;
use std::cmp::max;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let next = input.trim().parse::<i32>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let remain = input.trim().parse::<i32>().unwrap();

    let extra = max(next - (remain + 60), 0);

    let remains = (next - extra) * 1500;
    let extras = extra * 3000;

    println!("{}", remains + extras);
}
