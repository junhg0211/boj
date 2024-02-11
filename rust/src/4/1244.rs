use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let switch_count = input.trim().parse::<usize>().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let mut switches = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let student_count = input.trim().parse::<usize>().unwrap();

    for _ in 0..student_count {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());

        let gender = iter.next().unwrap();
        let number = iter.next().unwrap();

        if gender == 1 {
            let mut i = number-1;
            while i < switch_count {
                switches[i] = if switches[i] == 0 { 1 } else { 0 };
                i += number;
            }
        } else {
            let mut i = 1;

            switches[number-1] = if switches[number-1] == 0 { 1 } else { 0 };

            loop {
                if i > number-1 {
                    break;
                }

                let left = number-1 - i;
                let right = number-1 + i;

                if right >= switch_count {
                    break;
                }

                if switches[left] == switches[right] {
                    switches[left] = if switches[left] == 0 { 1 } else { 0 };
                    switches[right] = if switches[right] == 0 { 1 } else { 0 };
                } else {
                    break;
                }

                i += 1;
            }
        }
    }

    for (i, &state) in switches.iter().enumerate() {
        print!("{} ", state);
        if i % 20 == 19 {
            println!();
        }
    }
    println!();
}