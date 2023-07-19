use std::io::stdin;

fn main() {
    loop {
        // get number
        let mut number = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            input.trim().parse::<u32>().unwrap()
        };

        // terminal condition
        if number == 0 {
            break;
        }

        // calculate sum
        while number >= 10 {
            let mut sum = 0;
            while number > 0 {
                sum += number % 10;
                number /= 10;
            }
            number = sum;
        }

        // print result
        println!("{}", number);
    }
}
