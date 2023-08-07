use std::io::stdin;

fn main() {
    let (soongsil, korea, hanyang) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input.split_whitespace();

        (
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
        )
    };

    if soongsil + korea + hanyang >= 100 {
        println!("OK");
    } else if soongsil < korea && soongsil < hanyang {
        println!("Soongsil");
    } else if korea < soongsil && korea < hanyang {
        println!("Korea");
    } else {
        println!("Hanyang");
    }
}
