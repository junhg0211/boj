use std::io::stdin;

fn lerp(k: usize, i: usize, j: usize, a: u32, b: u32) -> f64 {
    let k = k as f64;
    let i = i as f64;
    let j = j as f64;
    let a = a as f64;
    let b = b as f64;

    return (k-i) / (j-i) * (b-a) + a;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let building_count = input
        .trim()
        .parse::<usize>()
        .unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let building_heights = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut max_visible_count = 0;
    for i in 0..building_count {  // 50
        let mut visible_count = 0;

        for j in 0..building_count { // 2,500
            if i == j {
                continue;
            }

            let mut not_visible = false;

            let (start, end) = if i < j {
                (i+1, j)
            } else {
                (j+1, i)
            };

            for k in start..end {
                if lerp(k, i, j, building_heights[i], building_heights[j]) <= building_heights[k] as f64 {
                    not_visible = true;
                    break;
                }
            }

            if not_visible {
                continue;
            }

            visible_count += 1;
        }

        if visible_count > max_visible_count {
            max_visible_count = visible_count;
            // println!("{}", i);
        }
    }

    println!("{}", max_visible_count);
}
