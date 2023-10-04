use std::io::stdin;
use std::collections::{ VecDeque, HashSet };

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());

    let subin = iter.next().unwrap();
    let sister = iter.next().unwrap();

    let mut queue = VecDeque::new();
    let mut beens = HashSet::new();
    queue.push_back((subin, 0));
    beens.insert(subin);

    while !queue.is_empty() {
        let (now, time) = queue.pop_front().unwrap();

        if now == sister {
            println!("{}", time);
            return;
        }

        if now*2 <= 100000 && !beens.contains(&(2*now)) {
            queue.push_back((2*now, time));
            beens.insert(2*now);
        }

        if now > 0 && !beens.contains(&(now-1)) {
            queue.push_back((now-1, time+1));
            beens.insert(now-1);
        }

        if now < 100000 && !beens.contains(&(now+1)) {
            queue.push_back((now+1, time+1));
            beens.insert(now+1);
        }
    }
}
