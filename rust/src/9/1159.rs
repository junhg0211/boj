use std::io::stdin;
use std::collections::HashMap;

fn main() {
    // get player count
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count: u32 = input.trim().parse().unwrap();

    // create hashmap for player name initial count
    let mut map: HashMap<char, u32> = HashMap::new();

    // loop for player names
    for _ in 0..count {
        // get player name
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let name = input.trim();
        let initial: char = name.chars().nth(0).unwrap();

        if map.contains_key(&initial) {
            // if player name initial is in hashmap, increase count
            let count = map.get_mut(&initial).unwrap();
            *count += 1;
        } else {
            // if player name initial is not in hashmap, add it
            map.insert(initial, 1);
        }
    }

    // iterate for key and value
    let mut chars: Vec<char> = Vec::new();
    for (key, value) in map.iter() {
        // if value >= 5, add key to list
        if value >= &5 {
            chars.push(*key);
        }
    }

    // sort list
    chars.sort();

    // print list
    if chars.len() > 0 {
        for c in chars {
            print!("{}", c);
        }
    } else {
        print!("PREDAJA");
    }
    println!();
}
