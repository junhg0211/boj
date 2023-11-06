use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let count = input.trim().parse::<usize>().unwrap();

    let mut points = Vec::new();
    for _ in 0..count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<f64>().unwrap());

        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        points.push((x, y));
    }

    let mut area = 0.0;
    for i in 0..count {
        let now = points[(i+1) % count];
        let previous = points[i];

        let dx = now.0 - previous.0;
        let dy = now.1 - previous.1;
        let local_min_y = if dy > 0.0 { previous.1 } else { now.1 };

        area += dx * (dy.abs() / 2.0 + local_min_y);
    }

    println!("{:.1}", area.abs());
}
