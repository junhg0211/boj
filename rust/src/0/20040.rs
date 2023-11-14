use std::io::stdin;
use std::collections::HashMap;

fn get_ancient(of: u32, map: &mut HashMap<u32, u32>) -> u32 {
    let mut this = of;
    while let Some(thing) = map.get(&this) {
        this = *thing;
    }
    if of != this {
        map.insert(of, this);
    }
    return this;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let _node_count = iter.next().unwrap().parse::<usize>().unwrap();
    let connection_count = iter.next().unwrap().parse::<usize>().unwrap();

    let mut parent = HashMap::new();
    for i in 0..connection_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        let a = iter.next().unwrap().parse::<u32>().unwrap();
        let b = iter.next().unwrap().parse::<u32>().unwrap();

        let aa = get_ancient(a, &mut parent);
        let ba = get_ancient(b, &mut parent);

        if aa == ba {
            println!("{}", i+1);
            return;
        }

        parent.insert(aa, ba);
    }

    println!("0");
}
