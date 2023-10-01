use std::io::stdin;
use std::collections::{ VecDeque, HashSet };

fn main() {
    // -- get commands
    let mut commands = Vec::new();
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let line = input.trim().to_owned();

        if &line == "." {
            break;
        }

        commands.push(line);
    }

    // -- check validity
    if commands.len() == 0 {
        println!("0");
        return;
    }

    // -- iterate
    let mut heads = VecDeque::new();
    let mut beens = HashSet::new();
    heads.push_back(0);
    beens.insert(0);

    while !heads.is_empty() {
        let now = heads.pop_front().unwrap();
        let command = &commands[now];

        // normal NOP
        if command.starts_with("RADI") {
            let dest = now + 1;

            if !beens.contains(&dest) && dest < commands.len() {
                heads.push_back(dest);
                beens.insert(dest);
            }

            continue;
        }

        // branch a or b
        if command.contains("ILI") {
            let mut iter = command[4..].split(" ILI ").map(|x| x.parse::<usize>().unwrap());
            let dest1 = iter.next().unwrap() - 1;
            let dest2 = iter.next().unwrap() - 1;

            if !beens.contains(&dest1) && 0 < dest1 && dest1 < commands.len() {
                heads.push_back(dest1);
                beens.insert(dest1);
            }

            if !beens.contains(&dest2) && 0 < dest2 && dest2 < commands.len() {
                heads.push_back(dest2);
                beens.insert(dest2);
            }

            continue;
        }

        // jump
        let dest = command[4..].parse::<usize>().unwrap() - 1;

        if !beens.contains(&dest) && 0 < dest && dest < commands.len() {
            heads.push_back(dest);
            beens.insert(dest);
        }
    }

    // -- print result
    println!("{}", commands.len() - beens.len());
}
