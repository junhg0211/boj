use std::io::stdin;

fn get_count() -> [i32; 26] {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut word1_count = [0; 26];

    for c in input.trim().chars() {
        word1_count[c as usize - 'a' as usize] += 1;
    }

    word1_count
}

fn main() {
    let count1 = get_count();
    let count2 = get_count();

    let mut result = 0;

    for i in 0..26 {
        result += (count1[i] - count2[i]).abs();
    }

    println!("{}", result);
}
