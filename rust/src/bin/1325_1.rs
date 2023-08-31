use std::io::stdin;
use std::collections::{ HashSet, VecDeque };

fn main() {
    let (computer_count, connection_count) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());

        (iter.next().unwrap(), iter.next().unwrap())
    };

    let mut connection = vec![HashSet::new(); computer_count];

    for _ in 0..connection_count {
        let (a, b) = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let mut iter = input
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap());

            (iter.next().unwrap(), iter.next().unwrap())
        };

        connection[a-1].insert(b);
    }

    let mut gots = vec![0; computer_count];

    for i in 1..computer_count+1 {
        let mut queue = VecDeque::from(vec![i]);

        while !queue.is_empty() {
            let now = queue.pop_front().unwrap();

            for connectee in connection.get(now-1).unwrap() {
                *gots.get_mut(connectee-1).unwrap() += 1;
                queue.push_back(*connectee);
            }
        }
    }

    let mut max_gots = 0;
    for i in gots.iter() {
        if *i > max_gots {
            max_gots = *i;
        }
    }

    for i in 0..computer_count {
        if gots[i] == max_gots {
            print!("{} ", i+1);
        }
    }
}
