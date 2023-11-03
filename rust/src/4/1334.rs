use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut number = input.trim().chars().collect::<Vec<_>>();

    // -- if 999...99
    let mut nines = true;
    for i in 0..number.len() {
        if number[i] != '9' {
            nines = false;
            break;
        }
    }

    if nines {
        print!("1");
        for _ in 0..number.len()-1 {
            print!("0");
        }
        println!("1");
        return;
    }

    // -- if palindrome already, increase one
    let mut palindrome = true;
    for i in 0..number.len()/2 {
        let ri = number.len()-1 - i;
        if number[i] != number[ri] {
            palindrome = false;
            break;
        }
    }
    if palindrome {
        let mut i = 0;
        loop {
            let ri = number.len()-1 - i;
            number[ri] = (number[ri] as u8 + 1) as char;
            if number[i] != ':' {
                break;
            }

            number[ri] = '0';
            i += 1;
        }
    }

    // -- increase
    loop {
        let limit = if number.len() % 2 == 0 {
            number.len() / 2
        } else {
            number.len() / 2 + 1
        };

        let mut same = true;
        for i in 0..limit {
            let front = number[i];
            let ri = number.len()-1 - i; // rear index
            let rear = number[ri];

            if front == rear {
                continue;
            }

            same = false;

            // 1321 -> 1331
            if front > rear {
                number[ri] = front;
                break;
            }

            // 12931 -> 13900
            let mut j = 1;
            loop {
                number[ri-j] = (number[ri-j] as u8 + 1) as char;
                if number[ri-j] != ':' {
                    break;
                }
                number[ri-j] = '0';
                j += 1;
            }
            number[ri] = '0';
            break;
        }

        if same {
            break;
        }
    }

    for c in number.iter() {
        print!("{}", c);
    }
    println!();
}
