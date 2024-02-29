use std::io::stdin;
use std::collections::HashSet;

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a < b {
        return gcd(b, a);
    }

    while b > 0 {
        (a, b) = (b, a % b);
    }

    return a;
}

fn lcm(a: usize, b: usize) -> usize {
    return a * b / gcd(a, b);
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let card_count = input.trim().parse::<usize>().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let target_cards = input.trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let shuffle = input.trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    // calculate lcm of path lengths
    let mut max_shuffle_count = 1;
    let mut beens: HashSet<usize> = HashSet::new();
    for mut i in 0..card_count {
        if beens.contains(&i) {
            continue;
        }

        let mut length = 1;
        loop {
            if beens.contains(&i) {
                break;
            }

            beens.insert(i);
            i = shuffle[i];
            length += 1;
        }

        max_shuffle_count = lcm(max_shuffle_count, length);
    }

    let mut hand = Vec::new();
    for i in 0..card_count {
        hand.push(i % 3);
    }

    let mut i = 0;
    loop {
        // check if returned
        let mut bad = true;
        for j in 0..card_count {
            if hand[j] != j % 3 {
                bad = false;
                break;
            }
        }
        if bad && i > 0 {
            println!("-1");
            return;
        }

        // check if good
        let mut good = true;
        for j in 0..card_count {
            if hand[j] != target_cards[j] {
                good = false;
                break;
            }
        }
        if good {
            println!("{}", i);
            return;
        }

        let mut new_hand = Vec::new();
        for j in 0..card_count {
            new_hand.push(hand[shuffle[j]]);
        }
        hand = new_hand;

        i += 1;
    }
}
