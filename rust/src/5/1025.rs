use std::io::stdin;

fn is_square(number: i32) -> bool {
    (number as f64).sqrt().round().powf(2.0) as i32 == number
}

fn main() {
    let (height, width) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input.split_whitespace();

        (
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap()
        )
    };

    let matrix = {
        let mut result = vec![vec![0i32; width as usize]; height as usize];

        for i in 0..height {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            for j in 0..width {
                result[i as usize][j as usize] = input.chars().nth(j as usize).unwrap().to_digit(10).unwrap() as i32;
            }
        }

        result
    };

    let mut result = -1;

    // check for all cells
    for i in 0..height {
    for j in 0..width {
        // generate numbers
        for dy in 0..=height as i32 { // 1, 4 quarters
        for dx in -(width as i32)..=width as i32 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let mut number = String::new();
            let mut k = 0;
            // generate number and check if it is a square
            while i + k * dy < height && j as i32 + k * dx < width {
                let y: usize = (i as i32 + k * dy) as usize;
                let x: usize = (j as i32 + k * dx) as usize;
                if y >= height as usize || x >= width as usize {
                    break;
                }

                // number = number * 10 + matrix[y][x];
                number.push_str(&matrix[y][x].to_string());
                k += 1;

                let number_parsed = number.parse().unwrap();
                if number_parsed > result && is_square(number_parsed) {
                    result = number_parsed;
                }
                // println!("({},{}), ({},{}), {}", j, i, dx, dy, number);

                let another = number.chars().rev().collect::<String>().parse().unwrap();
                if another > result && is_square(another) {
                    result = another;
                }
                // println!("({},{}), ({},{}), {}", j, i, dx, dy, another);
            }
        }
        }
    }
    }

    println!("{}", result);
}
