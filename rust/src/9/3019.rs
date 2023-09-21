use std::io::stdin;

fn main() {
    let (width, block) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());

        (
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    };

    let heights = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    };

    let possibles = vec![
        vec![ // I
            vec![0],
            vec![0, 0, 0, 0],
        ],
        vec![ // O
            vec![0, 0],
        ],
        vec![ // S
            vec![0, 0, 1],
            vec![1, 0],
        ],
        vec![ // Z
            vec![1, 0, 0],
            vec![0, 1],
        ],
        vec![ // T
            vec![0, 0, 0],
            vec![0, 1],
            vec![1, 0, 1],
            vec![1, 0],
        ],
        vec![ // L
            vec![0, 0, 0],
            vec![0, 0],
            vec![0, 1, 1],
            vec![2, 0],
        ],
        vec![ // J
            vec![0, 0, 0],
            vec![0, 2],
            vec![1, 1, 0],
            vec![0, 0],
        ]
    ];

    let shapes = &possibles[block - 1];
    let mut count = 0;
    for i in 0..width {
        for shape in shapes {
            if i + shape.len() > width {
                continue;
            }

            let gijun = heights[i] as i32 - shape[0] as i32;
            let mut fail = false;
            for j in 1..shape.len() {
                let this = heights[i+j] as i32 - shape[j] as i32;
                if this != gijun {
                    fail = true;
                    break;
                }
            }

            if !fail {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
