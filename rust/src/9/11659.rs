use std::io::{ stdin, stdout, BufWriter, Write };

fn main() {
    // --- get counts
    let (_number_count, interval_count) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());

        (
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    };

    // --- get sums
    let mut sums = vec![0];
    let mut previous = 0;
    for number in {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    } {
        previous += number;
        sums.push(previous);
    }

    // --- get intervals
    let mut writer = BufWriter::new(stdout());
    for _ in 0..interval_count {
        let (start, end) = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let mut iter = input
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap());

            (
                iter.next().unwrap(),
                iter.next().unwrap(),
            )
        };

        let result = sums[end] - sums[start-1];
        writeln!(writer, "{}", result).unwrap();
    }
}
