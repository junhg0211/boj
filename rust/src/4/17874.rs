use std::io::stdin;

fn main() {
    let (_, width, height) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input.split_whitespace();

        let size: u32 = iter.next().unwrap().parse().unwrap();
        let width: u32 = iter.next().unwrap().parse().unwrap();
        let height: u32 = iter.next().unwrap().parse().unwrap();

        let real_width = if size - width > width { size - width } else { width };
        let real_height = if size - height > height { size - height } else { height };

        (size, real_width, real_height)
    };

    println!("{}", width * height * 4);
}
