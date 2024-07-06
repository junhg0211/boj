use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    let mut cache = vec![0, 1, 1, 1 as u64];
    for i in 4..=n {
        cache.push(cache[i - 1] + cache[i - 3]);
    }

    println!("{}", cache[n]);
}
