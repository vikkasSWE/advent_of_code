// gamma: [0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0]
// epsilon: [1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1]
use std::fs;

fn main() {
    let file = "data.txt";
    let data = fs::read_to_string(file).unwrap();

    let content: Vec<&str> = data.split("\r\n").collect();
    let mut one_content: Vec<&str> = vec![];
    let mut two_content: Vec<&str> = vec![];
    let mut three_content: Vec<&str> = vec![];
    let mut four_content: Vec<&str> = vec![];
    let mut five_content: Vec<&str> = vec![];
    let mut six_content: Vec<&str> = vec![];
    let mut seven_content: Vec<&str> = vec![];
    let mut eight_content: Vec<&str> = vec![];
    let mut nine_content: Vec<&str> = vec![];
    let mut ten_content: Vec<&str> = vec![];
    let mut eleven_content: Vec<&str> = vec![];
    let mut twelve_content: Vec<&str> = vec![];
    // println!("{:?}", content);

    for element in content {
        if element.as_bytes()[0] == 49 {
            one_content.push(element);
        }
    }
    // gamma = [0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0];
    // epsilon = [1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1]

    for element in one_content {
        if element.as_bytes()[1] == 49 {
            two_content.push(element);
        }
    }
    for element in two_content {
        if element.as_bytes()[2] == 48 {
            three_content.push(element);
        }
    }
    for element in three_content {
        if element.as_bytes()[3] == 48 {
            four_content.push(element);
        }
    }
    for element in four_content {
        if element.as_bytes()[4] == 49 {
            five_content.push(element);
        }
    }
    for element in five_content {
        if element.as_bytes()[5] == 49 {
            six_content.push(element);
        }
    }
    for element in six_content {
        if element.as_bytes()[6] == 48 {
            seven_content.push(element);
        }
    }
    for element in seven_content {
        if element.as_bytes()[7] == 49 {
            eight_content.push(element);
        }
    }
    for element in eight_content {
        if element.as_bytes()[8] == 49 {
            nine_content.push(element);
        }
    }
    println!("OGR9: {:?}", nine_content);
    for element in nine_content {
        if element.as_bytes()[9] == 49 {
            ten_content.push(element);
        }
    }
    println!("OGR10: {:?}", ten_content);
    for element in ten_content {
        if element.as_bytes()[10] == 48 {
            eleven_content.push(element);
        }
    }
    println!("OGR11: {:?}", eleven_content);
    for element in eleven_content {
        if element.as_bytes()[11] == 49 {
            twelve_content.push(element);
        }
    }
    println!("OGR12: {:?}", twelve_content);

    // OGR: 100110010010
    // CO2: 111001101000
    println!("Answer: {}", 807 * 3289);
}
