use std::io::stdin;
use std::collections::{ BinaryHeap, VecDeque, HashMap, HashSet };
use std::cmp::max;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let target = iter.next().unwrap() as i32;
    let city_count = iter.next().unwrap();

    let mut cities = Vec::new();
    for _ in 1..=city_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());
        let price = iter.next().unwrap();
        let count = iter.next().unwrap();

        cities.push((price, count));
    }

    let mut heap = BinaryHeap::new();
    let mut member_count = HashMap::new();
    let mut beens = HashSet::new();
    heap.push(0);
    member_count.insert(0, 0);
    loop {
        let price = if let Some(thing) = heap.pop() {
            thing
        } else {
            break
        };
        let price = -price;
        let member = member_count.get(&price).unwrap();

        // println!("{} {} {}", price, member, heap.len());

        if member >= &target {
            println!("{}", price);
            return;
        }

        for (city_price, city_member) in cities.iter() {
            let count = *member_count.get(&price).unwrap_or(&0);
            let wow = *member_count.get(&(city_price + price)).unwrap_or(&0);

            let new_price = city_price + price;
            let new_member = max(count + city_member, wow);
            if !beens.contains(&(new_price, new_member)) {
                member_count.insert(new_price, new_member);
                heap.push(-(city_price + price));
            }

            beens.insert((new_price, new_member));
        }
    }
}
