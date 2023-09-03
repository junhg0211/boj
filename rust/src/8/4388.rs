use std::io::stdin;

fn main() {
    loop {
        let (mut a, mut b) = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let mut iter = input
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap());

            (iter.next().unwrap(), iter.next().unwrap())
        };

        if a == 0 && b == 0 {
            break;
        }

        let mut carry = 0;
        let mut carries = 0;
        while a > 0 || b > 0 {
            carry = (a%10 + b%10 + carry) / 10;
            a /= 10;
            b /= 10;

            if carry > 0 {
                carries += 1;
            }
        }

        println!("{}", carries);
    }
}
