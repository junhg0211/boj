use std::io::stdin;
use std::collections::{ VecDeque, HashSet };

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());

    let total_floors = iter.next().unwrap();
    let now = iter.next().unwrap();
    let target = iter.next().unwrap();
    let up_offset = iter.next().unwrap();
    let down_offset = iter.next().unwrap();

    let mut queue = VecDeque::new();
    let mut beens = HashSet::new();
    queue.push_back((now, 0));
    beens.insert(now);

    while !queue.is_empty() {
        let (now, length) = queue.pop_front().unwrap();

        if now == target {
            println!("{}", length);
            return;
        }

        let up = now + up_offset;
        let down = now - down_offset;

        if !beens.contains(&up) && up <= total_floors {
            queue.push_back((up, length + 1));
            beens.insert(up);
        }

        if !beens.contains(&down) && down >= 1 {
            queue.push_back((down, length + 1));
            beens.insert(down);
        }
    }

    println!("use the stairs");
}
