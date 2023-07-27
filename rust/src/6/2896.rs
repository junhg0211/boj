use std::io::stdin;

fn main() {
    let available = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<f64>().unwrap())
            .collect::<Vec<f64>>()
    };

    let ratio = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<f64>().unwrap())
            .collect::<Vec<f64>>()
    };

    // get unit amount
    let mut unit = f64::MAX;
    for i in 0..3 {
        let tmp = available[i] / ratio[i];
        if tmp < unit {
            unit = tmp;
        }
    }

    // print amount of each ingredient
    for i in 0..3 {
        print!("{} ", (available[i] - unit * ratio[i]));
    }
}
