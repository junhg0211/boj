use std::io::stdin;

fn tick(index: u32) {
    let place = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    print!("Case #{}: ", index);
    if place <= 25 {
        println!("World Finals");
    } else if place <= 1000 {
        println!("Round 3");
    } else if place <= 4500 {
        println!("Round 2");
    } else {
        println!("Round 1");
    }
}

fn main() {
    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    for i in 1..=count {
        tick(i);
    }
}
