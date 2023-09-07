use std::io::stdin;

fn main() {
    let (height, width) = {
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

    let mut place = vec![[0, 0]; 9];
    for _ in 0..height {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        for i in 0..width {
            let c = input.chars().nth(i as usize).unwrap();

            if '1' <= c && c <= '9' {
                let index = c as usize - '1' as usize;
                place[index][0] = index;
                place[index][1] = i as usize;
                break;
            }
        }
    }

    place.sort_by_key(|&x| -(x[1] as i32));

    let mut places = vec![0; 9];
    let mut previous_score = u32::MAX;
    let mut last_place = 0;
    for i in 0..9 {
        if previous_score > place[i][1] as u32 {
            last_place += 1;
            previous_score = place[i][1] as u32;
        }

        places[place[i][0] as usize] = last_place;
    }

    for p in places {
        println!("{}", p);
    }
}
