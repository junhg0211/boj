use std::io::stdin;
use std::cmp::min;

fn is_possible(mut number: i32, brokens: &Vec<i32>) -> bool {
    if number == 0 && brokens.contains(&0) {
        return false;
    }

    while number > 0 {
        if brokens.contains(&(number % 10)) {
            return false;
        }

        number /= 10;
    }

    return true;
}

fn get_places(mut number: i32) -> i32 {
    if number == 0 {
        return 1;
    }

    let mut result = 0;
    while number > 0 {
        result += 1;
        number /= 10;
    }
    return result;
}

fn main() {
    let target = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<i32>().unwrap()
    };

    let broken_count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<i32>().unwrap()
    };

    let brokens = if broken_count > 0 {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    } else { Vec::new() };

    if broken_count == 10 {
        println!("{}", (100 - target).abs());
        return;
    }

    let mut diff = 0;
    let result = loop {
        let mut possibles = Vec::new();

        if is_possible(target + diff, &brokens) {
            possibles.push(diff);
        }

        if target >= diff && is_possible(target - diff, &brokens) {
            possibles.push(-diff);
        }

        if possibles.len() == 1 {
            break possibles[0];
        }

        if possibles.len() == 2 {
            if get_places(possibles[0] + target) < get_places(possibles[1] + target) {
                break possibles[0];
            }

            break possibles[1];
        }

        diff += 1;
    };

    // println!("  {}", diff);

    let need = get_places(target + result) + diff.abs();
    println!("{}", min(need, (100 - target).abs()));
}
