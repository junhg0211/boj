use std::io::stdin;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap());

    let n = iter.next().unwrap();
    let p = iter.next().unwrap();
    let q = iter.next().unwrap();
    let x = iter.next().unwrap();
    let y = iter.next().unwrap();

    let mut map: HashMap<i64, i64> = HashMap::new();

    let mut stack = Vec::new();
    stack.push(n);
    while !stack.is_empty() {
        let now = stack.pop().unwrap();

        if map.contains_key(&now) {
            continue;
        }

        if now <= 0 {
            map.insert(now, 1);
            continue;
        }

        let a = now/p - x;
        if !map.contains_key(&a) {
            stack.push(now);
            stack.push(a);
            continue;
        }

        let b = now/q - y;
        if !map.contains_key(&b) {
            stack.push(now);
            stack.push(b);
            continue;
        }

        let result = map.get(&a).unwrap() + map.get(&b).unwrap();
        map.insert(now, result);
    }

    println!("{}", map.get(&n).unwrap());
}
