fn main() {
    for n in 2..=100 {
        let target = u32::pow(n, 3);

        for i in 2..target {
            let iii = u32::pow(i, 3);

            if iii > target {
                break;
            }

            for j in i+1..target {
                let jjj = u32::pow(j, 3);

                if iii + jjj > target {
                    break;
                }

                for k in j+1..target {
                    let kkk = u32::pow(k, 3);
                    let sum = iii + jjj + kkk;

                    if sum == target {
                        println!("Cube = {}, Triple = ({},{},{})", n, i, j, k);
                        break;
                    }

                    if sum > target {
                        break;
                    }
                }
            }
        }
    }
}
