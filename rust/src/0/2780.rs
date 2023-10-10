use std::io::stdin;
use std::collections::HashMap;

fn main() {
    let connections = HashMap::from([
        (0, vec![7]),
        (1, vec![2, 4]),
        (2, vec![1, 3, 5]),
        (3, vec![2, 6]),
        (4, vec![1, 5, 7]),
        (5, vec![2, 4, 6, 8]),
        (6, vec![3, 5, 9]),
        (7, vec![0, 4, 8]),
        (8, vec![5, 7, 9]),
        (9, vec![6, 8]),
    ]);

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let testcase_count = input.trim().parse::<usize>().unwrap();

    let mut testcases = Vec::new();
    let mut max_testcase = 0;
    for _ in 0..testcase_count {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let testcase = input.trim().parse::<usize>().unwrap();

        testcases.push(testcase);

        if testcase > max_testcase {
            max_testcase = testcase;
        }
    }

    let mut dp = vec![vec![0; 10]; max_testcase+1];

    for i in 0..10 {
        dp[1][i] = 1;
    }

    for i in 2..=max_testcase {
        for j in 0..10 {
            let mut result = 0;

            for connection in connections.get(&j).unwrap() {
                result += dp[i-1][*connection];
                result %= 1234567;
            }

            dp[i][j] = result;
        }
    }

    for testcase in testcases {
        let mut result = 0;

        for i in 0..10 {
            result += dp[testcase][i];
            result %= 1234567;
        }

        println!("{}", result);
    }
}
