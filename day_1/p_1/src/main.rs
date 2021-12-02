use std::fs;

fn main() {
    let file = "data.csv";
    let content = fs::read_to_string(file);

    let mut prev = 0;
    let mut count = 0;

    let content = content.unwrap();
    let word: Vec<&str> = content.split("\r\n").collect();

    for data in word {
        let curr = data.parse::<i32>().unwrap();

        if curr > prev {
            count = count + 1;
        }
        prev = curr;
    }

    // -1 BEACAUSE OF FIRST VALUE
    println!("Number of increses: {}", count - 1);
}
