use std::io::stdin;
use std::collections::{ HashMap, VecDeque };

/*
 * (100+1+|01)+
 * q0: 1 -> a0
 *     0 -> b0
 * a0: 0 -> a1
 * a1: 0 -> a1
 *     1 -> a2
 * a2: 1 -> q0, a2
 *     0 -> b0
 * b0: 1 -> q0
 *
 * (1000*11*|01)+
 * q0: 1 -> a0
 *     0 -> b0
 * a0: 0 -> a1
 * a1: 0 -> a2
 * a2: 0 -> a2, a3
 *     1 -> a3, q0
 * a3: 1 -> a3, q0
 * b0: 1 -> q0
 *
 * 10011001
 */

#[derive(Eq, Hash, PartialEq, Debug)]
enum State {
    Q0, B0, A0, A1, A2, A3
}
use State::*;

fn check(chars: &Vec<char>) -> bool {
    let mut connections = HashMap::new();
    connections.insert(Q0, vec![('1', &A0), ('0', &B0)]);
    connections.insert(A0, vec![('0', &A1)]);
    connections.insert(A1, vec![('0', &A2)]);
    connections.insert(A2, vec![('0', &A2), ('0', &A3), ('1', &A3), ('1', &Q0)]);
    connections.insert(A3, vec![('1', &Q0), ('1', &A3)]);
    connections.insert(B0, vec![('1', &Q0)]);

    let mut queue = VecDeque::new();
    queue.push_back((0, &Q0));

    loop {
        // println!("{:?}", queue);
        let (to_read_index, now_state) = match queue.pop_front() {
            Some(thing) => thing,
            None => break,
        };

        if to_read_index >= chars.len() {
            if now_state == &Q0 {
                return true;
            }

            continue;
        }

        let read_char = chars[to_read_index];
        let connection = connections.get(&now_state).unwrap();
        for (connected_char, next_state) in connection {
            if &read_char != connected_char {
                continue;
            }

            queue.push_back((to_read_index+1, *next_state));
        }
    }

    return false;
}

fn tick() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let message = input.trim().chars().collect::<Vec<_>>();

    match check(&message) {
        true => println!("YES"),
        false => println!("NO"),
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
