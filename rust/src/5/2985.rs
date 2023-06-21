use std::io::stdin;

fn main() {
    // get inputs
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let (a, b, c) = (numbers[0], numbers[1], numbers[2]);

    // print accordingly
    if a + b == c {
        println!("{}+{}={}", a, b, c);
    } else if a - b == c {
        println!("{}-{}={}", a, b, c);
    } else if a * b == c {
        println!("{}*{}={}", a, b, c);
    } else if a / b == c {
        println!("{}/{}={}", a, b, c);
    } else if a == b + c {
        println!("{}={}+{}", a, b, c);
    } else if a == b - c {
        println!("{}={}-{}", a, b, c);
    } else if a == b * c {
        println!("{}={}*{}", a, b, c);
    } else if a == b / c {
        println!("{}={}/{}", a, b, c);
    }
}
