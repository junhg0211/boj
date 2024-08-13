use std::{
    collections::HashMap,
    io::{stdin, stdout, BufWriter, Write},
};

fn w(a: i32, b: i32, c: i32, dp: &mut HashMap<(i32, i32, i32), i32>) -> i32 {
    if dp.contains_key(&(a, b, c)) {
        return *dp.get(&(a, b, c)).unwrap();
    }

    if a <= 0 || b <= 0 || c <= 0 {
        1
    } else if a > 20 || b > 20 || c > 20 {
        w(20, 20, 20, dp)
    } else if a < b && b < c {
        let value = w(a, b, c - 1, dp) + w(a, b - 1, c - 1, dp) - w(a, b - 1, c, dp);
        dp.insert((a, b, c), value);
        value
    } else {
        let value = w(a - 1, b, c, dp) + w(a - 1, b - 1, c, dp) + w(a - 1, b, c - 1, dp)
            - w(a - 1, b - 1, c - 1, dp);
        dp.insert((a, b, c), value);
        value
    }
}

fn main() {
    let mut writer = BufWriter::new(stdout());

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());

        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        let c = iter.next().unwrap();

        if a == b && b == c && c == -1 {
            break;
        }

        writeln!(
            writer,
            "w({}, {}, {}) = {}",
            a,
            b,
            c,
            w(a, b, c, &mut HashMap::new())
        )
        .unwrap();
    }
}
