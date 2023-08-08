use std::io::stdin;

fn main() {
    let word_count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<usize>().unwrap()
    };

    // what is the starting and ending
    let mut words = Vec::new();
    let mut the_index = 0;
    for i in 0..word_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        words.push(input.trim().to_string());

        if input.trim() == "?" {
            the_index = i;
        }
    }

    let starting = if the_index == 0 { ' ' } else { words[the_index-1].chars().last().unwrap() };
    let ending = if the_index == word_count-1 { ' ' } else { words[the_index+1].chars().next().unwrap() };

    // candidates
    let candidate_count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<usize>().unwrap()
    };

    let mut result = String::new();
    for _ in 0..candidate_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let word = input.trim();

        if (starting == ' ' || word.chars().next().unwrap() == starting)
                && (ending == ' ' || word.chars().last().unwrap() == ending)
                && !words.contains(&word.to_string()) {
            result.push_str(&word);
            continue;
        }
    }

    // print result
    println!("{}", result);
}
