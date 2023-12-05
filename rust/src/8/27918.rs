use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let round_count = input.trim().parse::<usize>().unwrap();

    let mut dalgoo = 0;
    let mut phoenix = 0;
    for _ in 0..round_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        if input.trim() == "D" {
            dalgoo += 1;
        } else {
            phoenix += 1;
        }

        if dalgoo - 2 == phoenix || phoenix - 2 == dalgoo {
            break;
        }
    }

    println!("{}:{}", dalgoo, phoenix);
}
