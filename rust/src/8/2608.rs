use std::io::stdin;

static NUMBERS: [(&str, u32); 13] = [
    ("M", 1000),
    ("CM", 900), ("D", 500), ("CD", 400), ("C", 100),
    ("XC", 90), ("L", 50), ("XL", 40), ("X", 10),
    ("IX", 9), ("V", 5), ("IV", 4), ("I", 1)];

fn roman_to_number(mut number: String) -> u32 {

    let mut result = 0;
    for (roman, weight) in &NUMBERS {
        while number.starts_with(roman) {
            result += weight;
            number = String::from(&number[roman.len()..]);
        }
    }

    return result;
}

fn number_to_roman(mut number: u32) -> String {
    let mut result = String::new();
    for (roman, weight) in &NUMBERS {
        while number >= *weight {
            result.push_str(roman);
            number -= weight;
        }
    }

    return result;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let number_1 = roman_to_number(input.trim().to_owned());

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let number_2 = roman_to_number(input.trim().to_owned());

    let result = number_1 + number_2;
    println!("{}", result);
    println!("{}", number_to_roman(result));
}
