use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let characters = input.trim().chars().collect::<Vec<_>>();

    let nth = characters[0] as usize - '0' as usize;
    let first = characters[characters.len() - 1] as usize - '0' as usize;
    let mut now = nth * first;

    let mut beens = vec![false; 100];

    loop {
        if beens[now] {
            println!("FA");
            break;
        }

        beens[now] = true;

        if now >= 10 {
            now = (now / 10) * (now % 10);
        } else {
            now = now * now;
        }
    }
}
