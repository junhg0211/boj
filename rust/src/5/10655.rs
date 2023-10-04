use std::io::stdin;

fn get_distance(from: (i32, i32), to: (i32, i32)) -> i32 {
    (from.0 - to.0).abs() + (from.1 - to.1).abs()
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let checkpoint_count = input.trim().parse::<usize>().unwrap();

    let mut checkpoints = Vec::new();
    let mut previous_position = (0, 0);
    let mut total_distance = 0;
    for i in 0..checkpoint_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());

        let checkpoint = (iter.next().unwrap(), iter.next().unwrap());
        checkpoints.push(checkpoint);

        // calculate total checkpoint distance
        if i != 0 {
            total_distance += get_distance(checkpoint, previous_position);
        }

        previous_position = checkpoint;
    }

    let mut result = i32::MAX;

    for i in 1..checkpoint_count-1 {
        let original = get_distance(checkpoints[i-1], checkpoints[i])
            + get_distance(checkpoints[i], checkpoints[i+1]);
        let skipped = get_distance(checkpoints[i-1], checkpoints[i+1]);

        let wow_distance = total_distance - original + skipped;

        if wow_distance < result {
            result = wow_distance;
        }
    }

    println!("{}", result);
}
