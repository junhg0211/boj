use std::io::stdin;

fn main() {
    let mut numbers = Vec::new();
    for _ in 0..4 {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let number = input
            .trim()
            .parse::<u32>().unwrap();
        numbers.push(number);
    }

    let mut increasing = true;
    let mut decreasing = true;
    let mut constant = true;

    for i in 1..4 {
        if numbers[i-1] >= numbers[i] {
            increasing = false;
        }
        if numbers[i-1] <= numbers[i] {
            decreasing = false;
        }
        if numbers[i-1] != numbers[i] {
            constant = false;
        }
    }

    if increasing {
        println!("Fish Rising");
    } else if decreasing {
        println!("Fish Diving");
    } else if constant {
        println!("Fish At Constant Depth");
    } else {
        println!("No Fish");
    }
}
