use std::io::stdin;

/**
 * 1. get the number of 1 in the binary representation of the number
 * 2. get the smallest number that can be added to the number
 * to make the number of 1 in the binary representation of the number
 * decrease by 1
 */
fn get_one_count(litre: u32) -> (u32, u32) {
    let mut anchor = 1;
    let mut result = 0;
    let mut smallest = 0;
    while anchor <= litre {
        if anchor & litre != 0 {
            result += 1;

            if smallest == 0 {
                smallest = anchor;
            }
        }

        anchor <<= 1;
    }
    return (result, smallest);
}

fn main() {
    // get numbers
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let having = numbers[0];
    let need = numbers[1];

    // calculate
    let mut result = 0;
    loop {
        let (ones, smallest) = get_one_count(having + result);

        if ones <= need {
            break;
        } else {
            result += smallest;
        }
    }

    println!("{}", result);
}
