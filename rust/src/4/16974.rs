use std::io::stdin;

fn length(level: u64) -> u64 {
    3 * ((1<<level) - 1) + (1<<level)
}

fn get_patties(level: u64, layer_count: u64) -> u64 {
    // println!("{} {}", level, layer_count);

    if level == 0 && layer_count > 0 {
        return 1;
    }

    let this_length = length(level);
    if layer_count >= this_length {
        return (1<<(level+1)) - 1;
    }

    let mut result = 0;

    // hamberger n-1
    if layer_count > 1 {
        result += get_patties(level-1, layer_count-1);
    }

    // patty
    let previous_length = length(level-1);
    if layer_count > previous_length+1 {
        result += 1;
    }

    // hamberget n-1
    if layer_count > previous_length+2 {
        result += get_patties(level-1, layer_count - (previous_length+2));
    }

    result
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap());

    let level = iter.next().unwrap();
    let layer_count = iter.next().unwrap();

    println!("{}", get_patties(level, layer_count));
}
