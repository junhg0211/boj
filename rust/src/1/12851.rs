use std::io::stdin;
use std::collections::{ VecDeque, HashMap };

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    let tnqls = iter.next().unwrap();
    let ehdtod = iter.next().unwrap();

    if tnqls == ehdtod {
        println!("0\n1");
        return;
    }

    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    queue.push_back((tnqls, 0));
    visited.insert(tnqls, 0);

    let mut fastest = u32::MAX;
    let mut fastest_count = 0;
    loop {
        let (now, time) = match queue.pop_front() {
            Some(thing) => thing,
            None => break,
        };

        // -- check if now == ehdtod
        if time > fastest {
            break;
        }

        if time == fastest && now != ehdtod {
            continue;
        }

        if now == ehdtod {
            fastest = time;
            fastest_count += 1;
            continue;
        }

        // -- next
        let mut visits = Vec::new();

        if now > 0 && visited.get(&(now-1)).unwrap_or(&u32::MAX) > &time {
            visits.push(now-1);
        }
        if now < 100000 && visited.get(&(now+1)).unwrap_or(&u32::MAX) > &time {
            visits.push(now+1);
        }
        if 2*now <= 100000 && visited.get(&(2*now)).unwrap_or(&u32::MAX) > &time {
            visits.push(2*now);
        }

        let next_time = time + 1;
        for next_now in visits {
            queue.push_back((next_now, next_time));
            visited.insert(next_now, next_time);
        }
    }

    println!("{}\n{}", fastest, fastest_count);
}
