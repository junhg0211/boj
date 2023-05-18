use std::io::stdin;

fn main() {
    let mut input = String::new();

    // antenna
    stdin().read_line(&mut input).unwrap();
    let antenna = input.trim().parse::<i32>().unwrap();

    // eyes
    input.clear();
    stdin().read_line(&mut input).unwrap();
    let eyes = input.trim().parse::<i32>().unwrap();

    if antenna >= 3 && eyes <= 4 {
        println!("TroyMartian");
    }

    if antenna <= 6 && eyes >= 2 {
        println!("VladSaturnian");
    }

    if antenna <= 2 && eyes <= 3 {
        println!("GraemeMercurian");
    }
}
