use std::io::stdin;

fn main() {
    let (mut year, mut month, mut day) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split("-")
            .map(|x| x.parse::<u32>().unwrap());

        (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    };

    let delta = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    day += delta;

    while day > 30 {
        month += 1;
        day -= 30;
    }

    while month > 12 {
        year += 1;
        month -= 12;
    }

    println!("{:04}-{:02}-{:02}", year, month, day);
}
