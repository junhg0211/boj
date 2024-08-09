use std::io::stdin;

fn recursion(s: Vec<char>, l: usize, r: usize, c: usize) -> (usize, usize) {
    if l >= r {
        (1, c)
    } else if s[l] != s[r] {
        (0, c)
    } else {
        recursion(s, l + 1, r - 1, c + 1)
    }
}

fn is_palindrome(s: Vec<char>) -> (usize, usize) {
    let length = s.len();
    recursion(s, 0, length - 1, 1)
}

fn tick() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let word = input.trim().chars().collect::<Vec<_>>();

    let result = is_palindrome(word);
    println!("{} {}", result.0, result.1);
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let word_count = input.trim().parse::<usize>().unwrap();

    for _ in 0..word_count {
        tick();
    }
}
