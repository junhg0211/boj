use std::io::stdin;

fn main() {
    let frame_count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<usize>().unwrap()
    };

    let recommendation_count = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<usize>().unwrap()
    };

    let recommendations = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    };

    let mut frames: Vec<[u32; 2]> = Vec::new();

    for i in 0..recommendation_count {
        let recommendation = recommendations[i];

        // check if candidate is in frame
        let mut good = false;
        for j in 0..frames.len() {
            if frames[j][0] == recommendation {
                frames[j][1] += 1;
                good = true;
                break;
            }
        }
        if good { continue; }

        // remove low rated oldest one
        if frames.len() == frame_count {
            let mut low_rated = Vec::new();
            let mut low_score = u32::MAX;

            for j in 0..frames.len() {
                let frame = frames[j];

                if frame[1] > low_score {
                    continue;
                }

                if frame[1] < low_score {
                    low_score = frame[1];
                    low_rated.clear();
                }

                low_rated.push(j);
            }

            frames.remove(low_rated[0]);
        }

        // insert new photo in a frame
        if frames.len() < frame_count {
            frames.push([recommendation, 1]);
            continue;
        }
    }

    let mut result = frames.iter().map(|&x| x[0]).collect::<Vec<_>>();
    result.sort();

    for number in result {
        print!("{} ", number);
    }
}
