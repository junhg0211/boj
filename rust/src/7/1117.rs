use std::io::stdin;
use std::cmp::{ min, max };

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap());

    let width = iter.next().unwrap();
    let height = iter.next().unwrap();
    let fold_x = iter.next().unwrap();
    let horizontal_fold_count = iter.next().unwrap();
    let x1 = iter.next().unwrap();
    let y1 = iter.next().unwrap();
    let x2 = iter.next().unwrap();
    let y2 = iter.next().unwrap();

    let fold_x = min(width-fold_x, fold_x);

    let part_fill_width = min(x2, max(x1, fold_x)) - x1;
    let out_width = max(x2, fold_x) - max(x1, fold_x);
    let part_height = y2 - y1;

    // println!("({}+{}) * {} / {}", part_fill_width, out_width, part_height, horizontal_fold_count);

    let filled_area = (part_fill_width * 2 + out_width) * part_height * (horizontal_fold_count+1);
    let result = height*width - filled_area;

    println!("{}", result);
}
