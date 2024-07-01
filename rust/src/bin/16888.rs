use std::{collections::HashMap, io::stdin};

fn tick(cache: &mut HashMap<u32, u32>) {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u32>().unwrap();

    let mut stack = vec![n];

    while !stack.is_empty() {
        let number = stack.pop().unwrap();

        if cache.contains_key(&number) {
            continue;
        }

        let mut i = (number as f32).sqrt() as u32;
        let mut pushed = false;
        let mut min = u32::MAX;
        while i > 0 {
            if i * i == number {
                min = 1;
                break;
            }

            if cache.contains_key(&(number - i * i)) {
                min = u32::min(min, cache.get(&(number - i * i)).unwrap() + 1);
                i -= 1;

                if min == 2 {
                    break;
                }
                continue;
            }

            if !pushed {
                stack.push(number);
                pushed = true;
            }
            if !cache.contains_key(&(number - i * i)) {
                stack.push(number - i * i);
                break;
            }
            i -= 1;
        }

        if !pushed {
            cache.insert(number, min);
        }
    }

    if cache.get(&n).unwrap() % 2 == 0 {
        println!("cubelover");
    } else {
        println!("koosaga");
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let testcase_count = input.trim().parse::<usize>().unwrap();

    let mut cache = HashMap::new();
    for _ in 0..testcase_count {
        tick(&mut cache);
    }
}
