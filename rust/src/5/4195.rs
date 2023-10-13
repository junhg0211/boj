use std::io::{ stdin, stdout, Write, BufWriter };
use std::collections::HashMap;

fn get_ancestor(of: String, parent: &mut HashMap<String, String>) -> String {
    let mut now = of.clone();
    let result = loop {
        match parent.get(&now) {
            Some(thing) => now = thing.to_string(),
            None => {
                break now;
            },
        }
    };

    if &of != &result {
        parent.insert(of, result.clone());
    }
    return result;
}

fn tick() {
    let mut writer = BufWriter::new(stdout());

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let relation_count = input.trim().parse::<usize>().unwrap();

    let mut parent = HashMap::new();
    let mut member_count = HashMap::new();
    for _ in 0..relation_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();

        let marco = iter.next().unwrap().to_owned();
        let polo = iter.next().unwrap().to_owned();

        let marco_papa = get_ancestor(marco, &mut parent);
        let polo_papa = get_ancestor(polo, &mut parent);

        if marco_papa == polo_papa {
            writeln!(writer, "{}", member_count.get(&marco_papa).unwrap()).unwrap();
            continue;
        }

        // println!("{} {}", marco_papa, polo_papa);
        parent.insert(polo_papa.clone(), marco_papa.clone());

        let polo_member_count = match member_count.get(&polo_papa) {
            Some(thing) => *thing,
            None => 1,
        };

        match member_count.get_mut(&marco_papa) {
            Some(thing) => {
                *thing += polo_member_count;
                writeln!(writer, "{}", thing).unwrap();
            },
            None => {
                member_count.insert(marco_papa, 1 + polo_member_count);
                writeln!(writer, "{}", 1 + polo_member_count).unwrap();
            },
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let testcase_count = input.trim().parse::<usize>().unwrap();
    for _ in 0..testcase_count {
        tick();
    }
}
