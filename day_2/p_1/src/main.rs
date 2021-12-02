use std::fs::{self, DirBuilder};

fn main() {
    let file = "data.txt";
    let content = fs::read_to_string(file).unwrap();
    let mut horizontal = 0;
    let mut depth = 0;

    for line in content.lines() {
        let mut data = line.split(' ');

        let dir = data.next().unwrap();
        let num = data.next().unwrap().parse::<i32>().unwrap();

        if dir == "forward" {
            horizontal = horizontal + num;
        } else if dir == "down" {
            depth = depth + num;
        } else if dir == "up" {
            depth = depth - num;
        }
    }

    println!("Depth: {}", depth);
    println!("Horizontal: {}", horizontal);

    println!("Number: {}", horizontal * depth);
}
