use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let wavelength = input.trim().parse::<u32>().unwrap();

    if wavelength < 425 {
        println!("Violet");
    } else if wavelength < 450 {
        println!("Indigo");
    } else if wavelength < 495 {
        println!("Blue");
    } else if wavelength < 570 {
        println!("Green");
    } else if wavelength < 590 {
        println!("Yellow");
    } else if wavelength < 620 {
        println!("Orange");
    } else if wavelength <= 780 {
        println!("Red");
    }
}
