use std::io::stdin;

fn main() {
    let number = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<f64>().unwrap()
    };

    let mut cache = [0.0f64; 1000];

    let remainder = number % 1.0;
    for i in 0..number as usize {
        cache[i] = if remainder + i as f64 <= 3f64 {
            1.0
        } else {
            cache[i-1] + cache[i-3]
        };
    }

    println!("{}", cache[number as usize - 1]);
}
