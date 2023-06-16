use std::io::stdin;

fn main() {
    // get count
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count: u32 = input.trim().parse().unwrap();

    for _ in 0..count {
        // get numbers
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let numbers: Vec<u32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let mut minimal = u32::MAX;
        let mut sum = 0 as u32;
        // iterate
        for number in numbers.iter() {
            let number = *number;
            if number % 2 == 0 {
                sum += number;
                if number < minimal {
                    minimal = number;
                }
            }
        }

        // print result
        println!("{} {}", sum, minimal);
    }
}
