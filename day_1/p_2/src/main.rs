use std::fs;

fn main() {
    let file = "data.csv";
    let content = fs::read_to_string(file);

    let mut prev = 0;
    let mut count = 0;

    let content = content.unwrap();
    let word: Vec<&str> = content.split("\r\n").collect();
    let data_int: Vec<i32> = word
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut sliding_data: Vec<i32> = vec![0];

    for i in 0..data_int.len() {
        if i < data_int.len() - 2 {
            sliding_data.push(data_int[i] + data_int[i + 1] + data_int[i + 2]);
        }
    }

    for data in sliding_data {
        let curr = data;
        if curr > prev {
            count = count + 1;
        }
        prev = curr;
    }

    // -1 BEACAUSE OF FIRST VALUE
    println!("Number of increses: {}", count - 1);
}
