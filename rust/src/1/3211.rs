use std::io::stdin;

fn main() {
    // get count of friends
    let friends = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    // get costs of friends
    let mut costs = Vec::new();
    for _ in 0..friends {
        let cost = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            input.trim().parse::<u32>().unwrap()
        };

        costs.push(cost);
    }
    costs.sort();

    // get the result count
    let mut result = 0;
    for i in 0..costs.len() {
        if i as u32 == costs[i] {
            result = i+1;
            break;
        }
    };

    println!("{}", result);
}
