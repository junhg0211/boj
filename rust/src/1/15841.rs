use std::io::stdin;

fn add(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    let mut carry = 0;
    for i in 0..usize::max(a.len(), b.len()) {
        let an = if i < a.len() { a[i] } else { 0 };
        let bn = if i < b.len() { b[i] } else { 0 };

        let o = an + bn + carry;
        result.push(o % 10);
        carry = o / 10;
    }

    if carry > 0 {
        result.push(carry);
    }

    result
}

fn stringify(number: &Vec<u8>) -> String {
    let mut result = String::new();

    for i in 0..number.len() {
        result.push(('0' as u8 + number[number.len() - i - 1]) as char);
    }

    result
}

fn main() {
    let mut cache = vec![vec![0], vec![1]];
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let string = input.trim();

        if string == "-1" {
            break;
        }

        let n = string.parse::<usize>().unwrap();

        while cache.len() <= n {
            let i = cache.len();
            cache.push(add(&cache[i - 1], &cache[i - 2]))
        }

        println!("Hour {}: {} cow(s) affected", n, stringify(&cache[n]));
    }
}
