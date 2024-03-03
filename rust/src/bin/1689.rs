use std::io::stdin;

fn linear_search(list: &Vec<i32>, number: i32, contains: bool) -> usize {
    let mut start = 0;
    let mut end = list.len();

    while start < end {
        let mut mid = (start + end) / 2;
        // println!("A {} {} {} {:?}", number, start, end, list);

        if number < list[mid] {
            end = mid;
            continue;
        }
        if list[mid] < number {
            if start == mid {
                return end;
            }

            start = mid;
            continue;
        }

        if contains {
            while mid < list.len() && list[mid] == number {
                mid += 1;
            }
        } else {
            while mid > 0 && list[mid-1] == number {
                mid -= 1;
            }
        }

        return mid;
    }

    return if contains { end } else { start };
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let line_count = input.trim().parse::<usize>().unwrap();

    let mut starts = Vec::new();
    let mut ends = Vec::new();
    for _ in 0..line_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim()
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap());

        let start = iter.next().unwrap();
        let end     = iter.next().unwrap();

        starts.push(start);
        ends.push(end);
    }
    starts.sort();
    ends.sort();

    // -- count
    let mut result = 0;
    for &point in starts.iter() {
        let point = point + 1;

        let start_count = linear_search(&starts, point, false) as i32;
        let end_count = linear_search(&ends, point, true) as i32;

        // println!("B {} {} {}", point, start_count, end_count);

        let value = start_count - end_count;
        if value > result {
            result = value;
        }
    }
    for &point in ends.iter() {
        let start_count = linear_search(&starts, point, false) as i32;
        let end_count = linear_search(&ends, point, true) as i32;

        // println!("C {} {} {}", point, start_count, end_count);

        let value = start_count - end_count;
        if value > result {
            result = value;
        }
    }

    println!("{}", result);
}
