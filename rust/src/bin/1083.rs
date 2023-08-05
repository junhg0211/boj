use std::io::stdin;
use std::cmp::min;

fn main() {
    // get variables
    let number_count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<usize>().unwrap()
    };
    let mut numbers: Vec<_> = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect()
    };

    let mut count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<u32>().unwrap()
    };

    // sort
    while count > 0 {
        // calculate where the descending sequence ends
        let mut sequence_end = 0;
        for i in 0..number_count - 1 {
            if numbers[i] < numbers[i + 1] {
                sequence_end = i + 1;
                break;
            }
        }

        // if the numbers are sorted, break
        if sequence_end == 0{
            break;
        }

        // calculate what is the biggest number after the sequence
        let mut biggest_index = sequence_end;
        for i in sequence_end..min(number_count, sequence_end + count as usize) {
            if numbers[i] > numbers[biggest_index] {
                biggest_index = i;
            }
        }

        // println!("{} {}", sequence_end, biggest_index);
        for i in (sequence_end..=biggest_index).rev() {
            numbers.swap(i - 1, i);
            // println!("{:?}", numbers);
            count -= 1;
        }
    }

    // print the result
    for number in numbers.iter() {
        print!("{} ", number);
    }
}
