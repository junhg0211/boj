use std::io::stdin;

fn main() {
    // get count
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<u8>().unwrap();

    // get sizes
    input.clear();
    stdin().read_line(&mut input).unwrap();
    let sizes: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    // get unit
    input.clear();
    stdin().read_line(&mut input).unwrap();
    let unit = input.trim().parse::<u64>().unwrap();

    // calculate size
    let mut result: u64 = 0;

    for i in 0..count {
        let size = sizes[i as usize];

        result += size / (unit as u64) + 1;
        if size % (unit as u64) == 0 {
            result -= 1;
        }
    }

    // print result
    println!("{}", result * unit);
}
