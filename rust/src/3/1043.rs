use std::io::stdin;
use std::collections::{ HashMap, HashSet };

fn get_ancestor(mut person: u32, parent: &HashMap::<u32, u32>) -> u32 {
    loop {
        match parent.get(&person) {
            Some(thing) => person = *thing,
            None => break person,
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());

    let _person_count = iter.next().unwrap();
    let party_count = iter.next().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    iter.next().unwrap();
    let trues = iter.collect::<Vec<_>>();

    let mut parent = HashMap::new();
    let mut parties = Vec::new();
    for _ in 0..party_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());

        let mut party = Vec::new();

        let party_people_count = iter.next().unwrap();
        let with = iter.next().unwrap();
        party.push(with);

        for _ in 0..party_people_count-1 {
            let p = iter.next().unwrap();
            party.push(p);

            let w_ancestor = get_ancestor(with, &parent);
            let p_ancestor = get_ancestor(p, &parent);

            if w_ancestor != p_ancestor {
                parent.insert(p_ancestor, w_ancestor);
            }
        }

        parties.push(party);
    }

    let mut true_ancestors = HashSet::new();
    for person in trues {
        true_ancestors.insert(get_ancestor(person, &parent));
    }

    let mut good_party_count = 0;
    for party in parties {
        let mut no = false;
        for member in party {
            if true_ancestors.contains(&get_ancestor(member, &parent)) {
                no = true;
                break;
            }
        }

        if no {
            continue;
        }

        good_party_count += 1;
    }

    println!("{}", good_party_count);
}
