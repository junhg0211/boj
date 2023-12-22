use std::io::stdin;
use std::collections::HashMap;

fn tick() {
    // -- get data from stdin
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let circle_count = iter.next().unwrap();
    let line_count = iter.next().unwrap();

    let mut connections: HashMap<usize, Vec<usize>> = HashMap::new();
    for _ in 0..line_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());
        let start = iter.next().unwrap() - 1;
        let end = iter.next().unwrap() - 1;

        if let Some(thing) = connections.get_mut(&start) {
            thing.push(end);
        } else {
            connections.insert(start, vec![end]);
        }

        if let Some(thing) = connections.get_mut(&end) {
            thing.push(start);
        } else {
            connections.insert(end, vec![start]);
        }
    }

    // -- wave function collapse
    let mut failed = false;
    let mut colors = vec![0; circle_count];
    for i in 0..circle_count {
        if colors[i] == 0 {
            colors[i] = 1;
            // println!("COLORED {} with {}", i+1, 1);
        }

        let result = fill(i, &connections, &mut colors);

        if result {
            failed = true;
            break;
        }
    }

    if failed {
        println!("impossible");
    } else {
        println!("possible");
    }
}

fn fill(i: usize, connections: &HashMap<usize, Vec<usize>>, colors: &mut Vec<u32>) -> bool {
    let reversed = reverse(colors[i]);
    for &connection in connections.get(&i).unwrap_or(&Vec::new()) {
        if colors[connection] == 0 {
            colors[connection] = reversed;
            fill(connection, connections, colors);
            // println!("COLORED {} with {}", connection+1, reversed);
        } else if colors[connection] != reversed {
            // println!("FAILED ON {} {}. {:?}", i+1, connection+1, colors);
            // println!("{} {} {}", colors[i], reversed, colors[connection]);
            return true;
        }
    }

    return false;
}

fn reverse(color: u32) -> u32 {
    if color == 1 { 2 } else { 1 }
}

fn main() {
    // get testcase count
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let testcase_count = input.trim().parse::<usize>().unwrap();

    // run testcase count times
    for _ in 0..testcase_count {
        tick();
    }
}
