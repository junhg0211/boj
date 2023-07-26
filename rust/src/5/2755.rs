use std::io::stdin;

fn get_point(grade: &str) -> f64 {
    match grade {
        "A+" => 4.3, "A0" => 4.0, "A-" => 3.7,
        "B+" => 3.3, "B0" => 3.0, "B-" => 2.7,
        "C+" => 2.3, "C0" => 2.0, "C-" => 1.7,
        "D+" => 1.3, "D0" => 1.0, "D-" => 0.7,
        _ => 0.0
    }
}

fn main() {
    // get count of subjects
    let count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input.trim().parse::<u32>().unwrap()
    };

    // calculate gpa
    let mut points = 0.0;
    let mut credits = 0.0;

    for _ in 0..count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input.trim().split_whitespace();

        let _name = iter.next().unwrap();
        let credit = iter.next().unwrap().parse::<f64>().unwrap();
        let grade = iter.next().unwrap();

        credits += credit;
        points += get_point(grade) * credit;
    }

    let gpa = points / credits as f64;

    // print result
    println!("{:.2}", (gpa * 100.0).round() / 100.0);
}
