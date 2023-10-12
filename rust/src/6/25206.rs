use std::io::stdin;

fn get_score_by_grade(grade: &str) -> f64 {
    match grade {
        "A+" => 4.5,
        "A0" => 4.0,
        "B+" => 3.5,
        "B0" => 3.0,
        "C+" => 2.5,
        "C0" => 2.0,
        "D+" => 1.5,
        "D0" => 1.0,
        _ => 0.0,
    }
}

fn main() {
    let mut count = 0.0;
    let mut sum = 0.0;

    for _ in 0..20 {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace();

        let _name = iter.next().unwrap();
        let weight = iter.next().unwrap().parse::<f64>().unwrap();
        let grade = iter.next().unwrap();
        let score = get_score_by_grade(grade);

        if grade == "P" {
            continue;
        }

        count += weight;
        sum += score * weight;
    }

    println!("{}", sum / count as f64);
}
