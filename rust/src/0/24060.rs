use std::io::stdin;

fn merge(a: &mut Vec<u32>, p: usize, q: usize, r: usize, k: &mut usize, tmp: &mut Vec<u32>) {
    let mut i = p;
    let mut j = q + 1;
    let mut t = 1;

    while i <= q && j <= r {
        if a[i - 1] <= a[j - 1] {
            tmp[t - 1] = a[i - 1];
            t += 1;
            i += 1;
        } else {
            tmp[t - 1] = a[j - 1];
            t += 1;
            j += 1;
        }
    }

    while i <= q {
        tmp[t - 1] = a[i - 1];
        t += 1;
        i += 1;
    }

    while j <= r {
        tmp[t - 1] = a[j - 1];
        t += 1;
        j += 1;
    }

    i = p;
    t = 1;

    while i <= r {
        a[i - 1] = tmp[t - 1];
        i += 1;
        t += 1;

        if *k > 0 {
            *k -= 1;
            if *k == 0 {
                println!("{}", a[i - 2]);
            }
        }
    }
}

fn merge_sort(a: &mut Vec<u32>, p: usize, r: usize, k: &mut usize, tmp: &mut Vec<u32>) {
    if p < r {
        let q = (p + r) / 2;
        merge_sort(a, p, q, k, tmp);
        merge_sort(a, q + 1, r, k, tmp);
        merge(a, p, q, r, k, tmp);
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let number_count = iter.next().unwrap();
    let mut k = iter.next().unwrap();

    input.clear();
    stdin().read_line(&mut input).unwrap();
    let mut vector = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut tmp = vec![0; number_count];
    merge_sort(&mut vector, 1, number_count, &mut k, &mut tmp);

    if k != 0 {
        println!("-1");
    }
}
