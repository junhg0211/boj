use std::io::stdin;

fn main() {
    let (_go, bus, subway) = {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        let mut iter = buf.split_whitespace();
        let go: u32 = iter.next().unwrap().parse().unwrap();
        let bus: u32 = iter.next().unwrap().parse().unwrap();
        let train: u32 = iter.next().unwrap().parse().unwrap();
        (go, bus, train)
    };

    if bus < subway {
        println!("Bus");
    } else if bus > subway {
        println!("Subway");
    } else {
        println!("Anything");
    }
}
