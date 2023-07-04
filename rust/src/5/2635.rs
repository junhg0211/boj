use std::io::stdin;

fn main() {
    // get first number
    let number = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<i32>().unwrap()
    };

    // iterate for 1..=number and get max sequence
    let mut sequence = Vec::<i32>::new();

    for i in 1..=number {
        let mut this_sequence = Vec::<i32>::new();
        this_sequence.push(number);
        this_sequence.push(i);

        loop {
            let new = {
                let length = this_sequence.len();
                this_sequence[length-2] - this_sequence[length-1]
            };

            if new < 0 {
                break;
            }

            this_sequence.push(new);
        }

        if this_sequence.len() > sequence.len() {
            sequence = this_sequence;
        }
    }

    // print result
    println!("{}", sequence.len());
    for number in sequence.iter() {
        print!("{} ", number);
    }
}
