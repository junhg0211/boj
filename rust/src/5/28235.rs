use std::io::stdin;

fn main() {
    println!("{}", match {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        &input.trim().to_string()[..]
    } {
        "SONGDO" => "HIGHSCHOOL",
        "CODE" => "MASTER",
        "2023" => "0611",
        _ => "CONTEST"
    });
}
