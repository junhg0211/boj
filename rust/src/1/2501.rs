use std::io::stdin;

fn main() {
    // get numbers
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", {
        let mut i = 1;
        let mut count = 0;
        loop {
            if numbers[0] % i == 0 {
                count += 1;

                if count == numbers[1] {
                    break i;
                }
            }

            if i > numbers[0] {
                break 0;
            }

            i += 1;
        }
    });
}
