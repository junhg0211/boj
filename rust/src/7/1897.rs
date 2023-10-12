use std::io::stdin;
use std::collections::{ HashMap, VecDeque, HashSet };

fn is_valid(wonjang_word: &str, dictionary_word: &str) -> bool {
    let mut i = 0;
    for letter in dictionary_word.chars() {
        if letter == wonjang_word.chars().nth(i).unwrap() {
            i += 1;

            if wonjang_word.len() == i {
                return true;
            }
        }
    }

    return false;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace();

    let dictionary_length = iter
        .next().unwrap()
        .parse::<usize>().unwrap();
    let wonjang_word = iter.next().unwrap().to_string();

    let mut words = HashMap::<usize, Vec<String>>::new();
    for _ in 0..dictionary_length {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let word = input.trim().to_owned();

        match words.get_mut(&word.len()) {
            Some(thing) => thing.push(word),
            None => {
                words.insert(word.len(), vec![word]);
            }
        }
    }

    let mut longest_word = String::new();

    let mut queue = VecDeque::new();
    let mut beens = HashSet::new();
    queue.push_back(wonjang_word.clone());
    beens.insert(wonjang_word);

    loop {
        stdin().read_line(&mut String::new()).unwrap();

        let word = match queue.pop_front() {
            Some(thing) => thing,
            None => break,
        };

        if word.len() > longest_word.len() {
            longest_word = word.to_string();
        }

        let possible_words = match words.get(&(word.len() + 1)) {
            Some(thing) => thing,
            None => continue,
        };

        for possible_word in possible_words {
            if beens.contains(&possible_word[..]) {
                continue;
            }

            if !is_valid(&word.clone(), possible_word) {
                continue;
            }

            queue.push_back(possible_word.clone());
            beens.insert(possible_word.clone());
        }
    }

    println!("{}", longest_word);
}
