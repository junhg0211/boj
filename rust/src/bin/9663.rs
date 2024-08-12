use std::io::stdin;

fn ok(position: usize, j: usize, i: usize, k: usize) -> bool {
    if position == i {
        return false;
    }
    if position + j == i + k {
        return false;
    }
    if position + k == i + j {
        return false;
    }

    true
}

fn backtrack(size: usize, count: usize, positions: &mut Vec<usize>) -> usize {
    let mut result = 0;

    for i in 0..size {
        let mut k = true;
        for (j, &position) in positions.iter().enumerate() {
            if !ok(position, j, i, size - count) {
                k = false;
                break;
            }
        }
        if !k {
            continue;
        }

        if count > 1 {
            positions.push(i);
            result += backtrack(size, count - 1, positions);
            positions.pop().unwrap();
        } else {
            result += 1;
        }
    }

    result
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let size = input.trim().parse::<usize>().unwrap();

    println!("{}", backtrack(size, size, &mut Vec::new()));
}
