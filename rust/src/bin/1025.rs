use std::io::stdin;

fn reverse_number(mut number: i32) -> i32 {
    let mut result = 0;
    while number > 0 {
        let digit = number % 10;
        result = result * 10 + digit;
        number /= 10;
    }
    result
}

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

            let mut number: i32 = 0;
            let mut k = 0;
            // generate number and check if it is a square
            while i + k * dy < height && j as i32 + k * dx < width {
                let y: usize = (i as i32 + k * dy) as usize;
                let x: usize = (j as i32 + k * dx) as usize;
                if y >= height as usize || x >= width as usize {
                    break;
                }

                number = number * 10 + matrix[y][x];
                k += 1;

                if number > result && is_square(number) {
                    result = number;
                }
                // println!("({},{}), ({},{}), {}", j, i, dx, dy, number);

                let another = reverse_number(number);
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
