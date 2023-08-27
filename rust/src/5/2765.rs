use std::{ io::stdin, f64::consts::PI };

fn main() {
    let mut index = 1;
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let numbers = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<f64>().unwrap())
            .collect::<Vec<_>>();

        let diameter = numbers[0];
        let rolls = numbers[1];
        let time = numbers[2];

        if rolls == 0.0 {
            break;
        }

        let distance = diameter * PI * rolls / 63360.0;
        let mph = distance / (time / 60.0 / 60.0);

        println!("Trip #{}: {:.02} {:.02}", index, distance, mph);

        // postloop
        index += 1;
    }
}
