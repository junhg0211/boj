use std::io::stdin;

fn main() {
    let mut singles = Vec::new();

    for _ in 0..5 {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input: u32 = input.trim().parse().unwrap();

        if singles.contains(&input) {
            singles.retain(|&x| x != input);
        } else {
            singles.push(input);
        }
    }

    println!("{}", singles.iter().sum::<u32>());
}
