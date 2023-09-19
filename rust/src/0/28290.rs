use std::io::stdin;

fn main() {
    let string = &{
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().to_owned()
    }[..];

    match string {
        "fdsajkl;" => println!("in-out"),
        "jkl;fdsa" => println!("in-out"),
        "asdf;lkj" => println!("out-in"),
        ";lkjasdf" => println!("out-in"),
        "asdfjkl;" => println!("stairs"),
        ";lkjfdsa" => println!("reverse"),
        _ => println!("molu"),
    }
}
