use std::io::stdin;

fn factorial(mut x: f64) -> f64 {
    let mut result = 1.0;
    while x > 0.0 {
        result *= x;
        x -= 1.0;
    }

    return result;
}

fn that(x: f64, y: f64) -> f64 {
    return factorial(x) / factorial(x-y);
}

fn ncr(n: f64, r: f64) -> f64 {
    return factorial(n) / (factorial(r) * factorial(n-r));
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap());

    let total_number_count = iter.next().unwrap();
    let selection_count = iter.next().unwrap();
    let threshold_count = iter.next().unwrap();

    let mut result = 0.0;
    let mut matching_count = threshold_count;
    while matching_count <= selection_count {
        let matches = that(selection_count, matching_count) / that(total_number_count, matching_count);
        let non_matches = that(total_number_count - selection_count, selection_count - matching_count)
            / that(total_number_count - matching_count, selection_count - matching_count);

        result += matches * non_matches * ncr(selection_count, matching_count);

        matching_count += 1.0;
    }

    println!("{:.10}", if result < 1.0 {
        result
    } else {
        1.0
    });
}