use std::io::stdin;

fn main() {
    // get variables
    let number_count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };
    let mut numbers: Vec<_> = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect()
    };

    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    // sort `count` times
    for _ in 0..count {
        let mut index = 0;
        let mut previous = 0;
        for i in 0..number_count as usize {
            if i > 0 && numbers[i] > previous {
                index = i;
                break;
            }

            previous = numbers[i];
        }

        if index == 0 {
            break;
        }

        // swap i-1 and i
        let tmp = numbers[index - 1];
        numbers[index - 1] = numbers[index];
        numbers[index] = tmp;

        println!("{:?}", numbers);
    }

    // print result
    for number in numbers.iter() {
        print!("{} ", number);
    }
}
