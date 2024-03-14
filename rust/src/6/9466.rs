use std::collections::HashSet;
use std::io::stdin;

fn tick() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let student_count = input.trim().parse::<usize>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let selections = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap() - 1)
        .collect::<Vec<_>>();

    let mut stack = Vec::new();
    let mut beens = HashSet::new();
    let mut circular_count = 0;
    let mut stack_beens = HashSet::new();
    for mut i in 0..student_count {
        stack.clear();
        stack_beens.clear();

        loop {
            if beens.contains(&i) {
                break;
            }

            stack.push(i);
            stack_beens.insert(i);
            let mut found = false;
            if stack_beens.contains(&selections[i]) {
                for j in 0..stack.len() {
                    if stack[j] == selections[i] {
                        circular_count += stack.len() - j;
                        beens.insert(i);
                        found = true;
                        // println!("+{} ({})", stack.len() - j, i);
                        break;
                    }
                }
                if found {
                    break;
                }
            }

            beens.insert(i);

            i = selections[i];
        }
    }

    // println!("{}", circular_count);
    println!("{}", student_count - circular_count);
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let testcase_count = input.trim().parse::<usize>().unwrap();

    for _ in 0..testcase_count {
        tick();
    }
}
