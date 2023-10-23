use std::io::stdin;
use std::collections::{ HashMap, BinaryHeap };

fn get_ancient(mut node: usize, parent_map: &HashMap<usize, usize>) -> usize {
    loop {
        node = match parent_map.get(&node) {
            Some(thing) => *thing,
            None => return node,
        };
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let node_count = input.trim().parse::<usize>().unwrap();

    // -- get edges
    let mut edges = BinaryHeap::new();
    let mut parent = HashMap::new();
    let mut price = 0;
    for i in 0..node_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let weights = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());

        for (j, weight) in weights.enumerate() {
            if weight > 0 {
                edges.push((-weight, i, j));
                continue;
            }

            price -= weight;

            if i == j {
                continue;
            }

            let i_ancient = get_ancient(i, &parent);
            let j_ancient = get_ancient(j, &parent);

            if i_ancient == j_ancient {
                continue;
            }

            parent.insert(i_ancient, j_ancient);
        }
    }
    price /= 2;

    // -- kruskal
    let mut new_edges = Vec::new();
    loop {
        let (negative_weight, i, j) = match edges.pop() {
            Some(thing) => thing,
            None => break,
        };

        let weight = -negative_weight;

        let i_ancient = get_ancient(i, &parent);
        let j_ancient = get_ancient(j, &parent);

        if i_ancient == j_ancient {
            continue;
        }

        parent.insert(i_ancient, j_ancient);
        new_edges.push((i, j));
        price += weight;
    }

    // -- print result
    println!("{} {}", price, new_edges.len());
    for (i, j) in new_edges {
        println!("{} {}", j+1, i+1);
    }
}
