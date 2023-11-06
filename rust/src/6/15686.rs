use std::io::stdin;

fn distance(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn get_chicken_distance(
    houses: Vec<(usize, usize)>,
    chickens: Vec<(usize, usize)>,
    chicken_exclude_count: usize,
    from_index: usize
) -> usize {
    if chicken_exclude_count == 0 {
        let mut result = 0;
        for house in houses.clone() {
            let mut min = usize::MAX;
            for chicken in chickens.clone() {
                let distance = distance(house.0, chicken.0) + distance(house.1, chicken.1);

                if distance < min {
                    min = distance;
                }
            }

            result += min;
        }

        return result;
    }

    let mut min = usize::MAX;
    for i in from_index..chickens.len() {
        let mut new_chickens = chickens.clone();
        new_chickens.remove(i);

        let distance = get_chicken_distance(houses.clone(), new_chickens, chicken_exclude_count-1, i);

        if distance < min {
            min = distance;
        }
    }

    return min;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let size = iter.next().unwrap();
    let max_chicken_count = iter.next().unwrap();

    let mut houses = Vec::new();
    let mut chickens = Vec::new();
    for i in 0..size {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        for (j, content) in input.trim().split_whitespace().enumerate() {
            if content == "1" {
                houses.push((i, j));
            } else if content == "2" {
                chickens.push((i, j));
            }
        }
    }

    let result = get_chicken_distance(houses, chickens.clone(), chickens.len() - max_chicken_count, 0);
    println!("{}", result);
}
