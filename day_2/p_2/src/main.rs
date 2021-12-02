use std::fs;

fn main() {
    let file = "data.txt";
    let content = fs::read_to_string(file).unwrap();
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in content.lines() {
        let mut data = line.split(' ');

        let dir = data.next().unwrap();
        let num = data.next().unwrap().parse::<i32>().unwrap();

        if dir == "forward" {
            horizontal = horizontal + num;

            depth = depth + aim * num;
        } else if dir == "down" {
            aim = aim + num;
        } else if dir == "up" {
            aim = aim - num;
        }
    }

    println!("Depth: {}", depth);
    println!("Horizontal: {}", horizontal);

    println!("Number: {}", horizontal * depth);
}
