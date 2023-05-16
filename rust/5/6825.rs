use std::io;

fn main() {
    let mut input = String::new();

    // get weight
    io::stdin().read_line(&mut input).unwrap();
    let weight = input.trim().parse::<f64>().unwrap();
    input = "".to_string();

    // get height
    io::stdin().read_line(&mut input).unwrap();
    let height = input.trim().parse::<f64>().unwrap();

    let bmi = weight / (height * height);

    if bmi > 25.0 {
        println!("Overweight");
    } else if bmi >= 18.5 {
        println!("Normal weight");
    } else {
        println!("Underweight");
    }
}
