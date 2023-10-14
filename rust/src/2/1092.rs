use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let crane_count = input.trim().parse::<usize>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut crane_limits = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    crane_limits.sort_by_key(|x| -(*x as i32));

    let mut max_limit = 0;
    for limit in &crane_limits {
        if limit > &max_limit {
            max_limit = *limit;
        }
    }

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let _box_count = input.trim().parse::<usize>().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut box_weights = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    box_weights.sort_by_key(|x| -(*x as i32));

    // check validity
    for box_weight in &box_weights {
        if box_weight > &max_limit {
            println!("-1");
            return;
        }
    }

    // count movings
    let mut result = 0;
    loop {
        let mut deleteds = Vec::new();

        let mut now_crane_index = 0;
        for (i, box_weight) in box_weights.iter().enumerate() {
            if box_weight <= &crane_limits[now_crane_index] {
                now_crane_index += 1;
                deleteds.push(i);

                if now_crane_index >= crane_count {
                    break;
                }
            }
        }
        result += 1;

        let mut offset = 0;
        for deleted in deleteds {
            box_weights.remove(deleted - offset);
            offset += 1;
        }

        if box_weights.is_empty() {
            break;
        }
    }

    println!("{}", result);
}
