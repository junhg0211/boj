use std::io::stdin;

fn main() {
    let mut min = u32::MIN;
    let mut max = u32::MAX;

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let number = input.trim().parse::<u32>().unwrap();

        if number == 0 {
            return;
        }

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let info = input.trim().to_string();

        if info == String::from("too high") {
            max = if number < max { number } else { max };
        } else if info == String::from("too low") {
            min = if number > min { number } else { min };
        } else {
            // println!("{} {}", min, max);

            if min < max && min < number && number < max {
                println!("Stan may be honest");
            } else {
                println!("Stan is dishonest");
            }

            min = u32::MIN;
            max = u32::MAX;
        }
    }
}
