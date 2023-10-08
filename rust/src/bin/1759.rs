use std::io::stdin;

fn f(letters: &Vec<char>, length: usize, from_index: usize, vowels: bool, stack: &mut Vec<char>) {
    if length == 0 {
        return;
    }

    for i in from_index..letters.len() {
        let letter = letters[i];
        let vowel = String::from("aeiou").contains(letter);

        if (vowel || vowels) && length == 1 {
            for c in &mut *stack {
                print!("{}", c);
            }
            println!("{}", letter);
            continue;
        }

        stack.push(letter);
        f(letters, length-1, i+1, vowels || vowel, stack);
        stack.pop().unwrap();
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let length = iter.next().unwrap();
    let _character_count = iter.next().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut letters = input
        .trim()
        .split_whitespace()
        .map(|x| x.chars().next().unwrap())
        .collect::<Vec<_>>();
    letters.sort();

    let mut stack = Vec::new();
    f(&letters, length, 0, false, &mut stack);
}
