use std::io::stdin;

fn main() {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut sides = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        sides.sort();

        if sides[2] == 0 {
            break;
        }

        if sides[2] >= sides[0] + sides[1] {
            println!("Invalid");
        } else if sides[0] == sides[1] && sides[1] == sides[2] {
            println!("Equilateral");
        } else if sides[0] == sides[1] || sides[1] == sides[2] {
            println!("Isosceles");
        } else {
            println!("Scalene");
        }
    }
}
