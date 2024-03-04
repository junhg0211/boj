use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap());
    let start = iter.next().unwrap();
    let end = iter.next().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<f64>().unwrap());
    let divisor = iter.next().unwrap();
    let remainder = iter.next().unwrap();

    if divisor == 0.0 {
        println!("Unknwon Number");
        return;
    }

    let limit1 = (start - remainder) / divisor;
    let limit2 = (end - remainder) / divisor;

    let (limit1, limit2) = if limit1 > limit2 {
        (limit2, limit1)
    } else {
        (limit1, limit2)
    };

    println!("{} {}", limit1, limit2);

    if limit1.ceil() == limit2.floor() {
        println!("{}", limit1.ceil() * divisor + remainder);
    } else {
        println!("Unknwon Number");
    }
}
