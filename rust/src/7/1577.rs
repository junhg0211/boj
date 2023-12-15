use std::io::stdin;
use std::collections::{ HashMap, HashSet };

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let height = iter.next().unwrap();
    let width = iter.next().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let construction_count = input.trim().parse::<usize>().unwrap();
    let mut constructions: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
    for _ in 0..construction_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());
        let y1 = iter.next().unwrap();
        let x1 = iter.next().unwrap();
        let y2 = iter.next().unwrap();
        let x2 = iter.next().unwrap();

        if constructions.contains_key(&(y1, x1)) {
            constructions.get_mut(&(y1, x1)).unwrap().insert((y2, x2));
        } else {
            let mut set = HashSet::new();
            set.insert((y2, x2));
            constructions.insert((y1, x1), set);
        }

        if constructions.contains_key(&(y2, x2)) {
            constructions.get_mut(&(y2, x2)).unwrap().insert((y1, x1));
        } else {
            let mut set = HashSet::new();
            set.insert((y1, x1));
            constructions.insert((y2, x2), set);
        }
    }

    let mut possible_count = vec![vec![0; width+1]; height+1];
    possible_count[0][0] = 1;
    for i in 0..=height {
        for j in 0..=width {
            if i == 0 && j == 0 {
                continue;
            }

            let mut result = 0;

            if i > 0 && !constructions
                    .get(&(i, j))
                    .unwrap_or(&HashSet::new())
                    .contains(&(i-1, j)) {
                result += possible_count[i-1][j] as u64;
            }

            if j > 0 && !constructions
                    .get(&(i, j))
                    .unwrap_or(&HashSet::new())
                    .contains(&(i, j-1)) {
                result += possible_count[i][j-1] as u64;
            }

            possible_count[i][j] = result;
        }
    }

    // println!("{:?}", possible_count);
    println!("{}", possible_count[height][width]);
}
