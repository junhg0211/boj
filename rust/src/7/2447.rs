use std::io::{ stdin, stdout, Write, BufWriter };

fn is_star(x: i32, y: i32, iteration: i32) -> bool {
    if x % 3 == 1 && y % 3 == 1 {
        return false;
    }

    if iteration == 0 {
        return true;
    }

    return is_star(x / 3, y / 3, iteration - 1);
}

fn main() {
    let size = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<i32>().unwrap()
    };

    let iterations = (size as f64).log(3.0).floor() as i32;

    let mut writer = BufWriter::new(stdout());
    for y in 0..size {
        for x in 0..size {
            write!(writer, "{}", if is_star(x, y, iterations) { "*" } else { " " }).unwrap();
        }
        writeln!(writer).unwrap();
    }
}
