use std::io::{ stdin, stdout, Write, BufWriter };
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

    let mut queue = VecDeque::new();
    let mut previous = HashMap::new();
    queue.push_front(tnqls);
    previous.insert(tnqls, tnqls);

    loop {
        let now = queue.pop_back().unwrap();

        if now == ehdtod {
            break;
        }

        if now > 0 && !previous.contains_key(&(now-1)) {
            queue.push_front(now-1);
            previous.insert(now-1, now);
        }
        if now < 100000 && !previous.contains_key(&(now+1)) {
            queue.push_front(now+1);
            previous.insert(now+1, now);
        }
        if 2*now <= 100000 && !previous.contains_key(&(2*now)) {
            queue.push_front(2*now);
            previous.insert(2*now, now);
        }
    };

    // print result
    let mut writer = BufWriter::new(stdout());

    let mut stack = Vec::new();
    let mut now = ehdtod;
    while now != tnqls {
        stack.push(now);
        now = previous[&now];
    }

    writeln!(writer, "{}", stack.len()).unwrap();
    write!(writer, "{} ", tnqls).unwrap();
    loop {
        match stack.pop() {
            Some(thing) => write!(writer, "{} ", thing).unwrap(),
            None => break,
        }
    }
    writeln!(writer).unwrap();
}

