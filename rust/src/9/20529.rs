use std::io::stdin;

fn mbti_to_number(mbti: &str) -> usize {
    let mut result = 0;

    if mbti.chars().nth(0).unwrap() == 'I' {
        result += 8;
    }
    if mbti.chars().nth(1).unwrap() == 'S' {
        result += 4;
    }
    if mbti.chars().nth(2).unwrap() == 'T' {
        result += 2;
    }
    if mbti.chars().nth(3).unwrap() == 'P' {
        result += 1;
    }

    result
}

fn get_social_distance(a: usize, b: usize) -> u32 {
    let mut result = 0;

    let mut diff = a ^ b;
    while diff > 0 {
        if diff & 1 == 1 {
            result += 1;
        }
        diff >>= 1;
    }

    return result;
}

fn tick() {
    // get member_count
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let member_count = input.trim().parse::<usize>().unwrap();

    // get mbtis
    input.clear();
    stdin().read_line(&mut input).unwrap();
    let mbtis = input.trim().split_whitespace().map(|x| mbti_to_number(x)).collect::<Vec<_>>();

    // if duplicated mbtis are found, skip the testcase
    if member_count > 16*2 {
        println!("0");
        return;
    }

    // if a duplicated mbti is found, skip the testcase
    let mut counts = [0; 16];
    for mbti in &mbtis {
        counts[*mbti] += 1;

        if counts[*mbti] >= 3 {
            println!("0");
            return;
        }
    }

    // calculate social distance
    let mut result = 12;
    for i in 0..member_count-2 {
        for j in i+1..member_count-1 {
            for k in j+1..member_count {
                let social_distance = get_social_distance(mbtis[i], mbtis[j])
                    + get_social_distance(mbtis[i], mbtis[k])
                    + get_social_distance(mbtis[j], mbtis[k]);

                if social_distance < result {
                    result = social_distance;
                }
            }
        }
    }

    println!("{}", result);
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let testcase_count = input.trim().parse::<u32>().unwrap();

    for _ in 0..testcase_count {
        tick();
    }
}
