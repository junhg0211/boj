use std::io::stdin;

fn main() {
    // --- get input
    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<usize>().unwrap()
    };

    let platforms = {
        let mut result = Vec::new();

        for _ in 0..count {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let mut iter = input
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap());

            result.push((
                iter.next().unwrap(),
                iter.next().unwrap(),
                iter.next().unwrap(),
            ));
        }

        result.sort_by_key(|&x| -(x.0 as i32));
        result
    };

    // --- calculate pillars
    let mut result = 0;
    for i in 0..count {
        let (y, x1, x2) = platforms[i];

        let mut left_ok = false;
        let mut right_ok = false;

        for j in i+1..count {
            let (ly, lx1, lx2) = platforms[j];

            // left pillar
            if !left_ok && lx1 <= x1 && x1 < lx2 {
                result += y - ly;
                left_ok = true;

                // println!("{} {} {}", i, j, y-ly);
            }

            // right pillar
            if !right_ok && lx1 < x2 && x2 <= lx2 {
                result += y - ly;
                right_ok = true;

                // println!("{} {} {}", i, j, y-ly);
            }

            if left_ok && right_ok {
                break;
            }
        }

        if !left_ok {
            result += y;

            // println!("{} {}", i, y);
        }

        if !right_ok {
            result += y;

            // println!("{} {}", i, y);
        }
    }

    println!("{}", result);
}
