use std::io::stdin;
use std::cmp::min;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut operatees = Vec::new();
    for c in input.trim().chars().rev() {
        match c {
            'x' => operatees.push(0),
            'g' => {
                if operatees.len() < 1 {
                    println!("-1");
                    return;
                }

                let pop = operatees.pop().unwrap();
                operatees.push(pop + 1);
            },
            'f' => {
                if operatees.len() < 2 {
                    println!("-1");
                    return;
                }

                let a = operatees.pop().unwrap();
                let b = operatees.pop().unwrap();
                operatees.push(min(a, b));
            },
            _ => (),
        }
    }

    if operatees.len() != 1 {
        println!("-1");
        return;
    }

    println!("{}", operatees.pop().unwrap());
}
