use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<usize>().unwrap();

    let mut stack: Vec<(i32, Vec<(i32, String)>)> = Vec::new();
    stack.push((0, Vec::new()));
    for _ in 0..count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();

        if iter.next().unwrap() == "type" {
            let letter = iter.next().unwrap();
            let time = iter.next().unwrap().parse::<i32>().unwrap();

            let last = stack.last().unwrap().1.to_owned();
            let new = last.clone() + letter;

            stack.push((time, new));
        } else {
            let amount = iter.next().unwrap().parse::<i32>().unwrap();
            let time = iter.next().unwrap().parse::<i32>().unwrap();

            let then = time - amount;
            while let Some(thing) = stack.pop() {
                if thing.0 <= then {
                    break;
                }
            }
        }

        println!("{:?}", stack);
    }

    println!("{}", stack.pop().unwrap().1);
}
