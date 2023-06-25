use std::io::stdin;

fn main() {
    // input buffer
    let mut input = String::new();

    // session
    loop {
        // get area count
        let count = {
            input.clear();
            stdin().read_line(&mut input).unwrap();
            input.trim().parse::<i32>().unwrap()
        };

        // exit condition
        if count == -1 {
            break;
        }

        // calculation
        let mut previous = 0;
        let mut gone = 0;
        for _ in 0..count {
            let (speed, time) = {
                input.clear();
                stdin().read_line(&mut input).unwrap();
                let mut iter = input.trim().split_whitespace();

                (
                    iter.next().unwrap().parse::<i32>().unwrap(),
                    iter.next().unwrap().parse::<i32>().unwrap(),
                )
            };

            gone += speed * (time - previous);
            previous = time;
        }

        println!("{} miles", gone);
    }
}
