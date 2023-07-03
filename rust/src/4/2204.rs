use std::io::stdin;

fn main() {
    loop {
        // get count of words
        let count = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            input.trim().parse::<u32>().unwrap()
        };

        // terminal condition
        if count == 0 {
            break;
        }

        // result = min(input words)
        let mut result = String::new();
        for i in 0..count {
            // get word
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let word = input.trim().to_string();

            // if first, result = word
            if i == 0 {
                result = word;
                continue;
            }

            // if not first, compare result and word
            if result.to_lowercase() > word.to_lowercase() {
                result = word;
            }
        }

        // print result
        println!("{}", result);
    }
}
