use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut m = false;
    let mut o = false;
    let mut b = false;
    let mut i = false;
    let mut s = false;

    for c in input.chars() {
        match c {
            'M' => m = true,
            'O' => o = true,
            'B' => b = true,
            'I' => i = true,
            'S' => s = true,
            _ => (),
        }
    }

    if m && o && b && i && s {
        println!("YES");
    } else {
        println!("NO");
    }
}
