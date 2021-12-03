use std::fs;

fn main() {
    let file = "data.txt";
    let data = fs::read_to_string(file).unwrap();

    let mut gamma_zero: Vec<i32> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut gamma_one: Vec<i32> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut gamma: Vec<i32> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut epsilon: Vec<i32> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for line in data.lines() {
        let len = line.as_bytes().len();
        for i in 0..len - 1 {
            if line.as_bytes()[i] == 48 {
                gamma_zero[i] = gamma_zero[i] + 1;
            } else {
                gamma_one[i] = gamma_one[i] + 1;
            }
        }
    }
    for i in 0..12 {
        if gamma_one[i] > gamma_zero[i] {
            gamma[i] = 1;
            epsilon[i] = 0;
        } else {
            gamma[i] = 0;
            epsilon[i] = 1;
        }
    }

    println!("{:?}", gamma); // 802
    println!("{:?}", epsilon); // 3293

    println!("Answer: {}", 802 * 3293);
}
