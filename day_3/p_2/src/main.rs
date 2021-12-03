// gamma: [0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0]
// epsilon: [1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1]
use std::fs;

fn calculate<'a>(content: Vec<&'a str>, index: usize) -> Vec<&'a str> {
    let mut gamma_zero = 0;
    let mut gamma_one = 0;

    for line in content.clone() {
        if line.as_bytes()[index] == 48 {
            gamma_zero = gamma_zero + 1;
        } else {
            gamma_one = gamma_one + 1;
        }
    }

    let one_or_zero;
    // Set true for OGR
    if false {
        if gamma_one > gamma_zero {
            one_or_zero = false;
        } else if gamma_one < gamma_zero {
            one_or_zero = true;
        } else {
            one_or_zero = false;
        }
    } else {
        if gamma_one > gamma_zero {
            one_or_zero = true;
        } else if gamma_one < gamma_zero {
            one_or_zero = false;
        } else {
            one_or_zero = true;
        }
    }

    return content_calc(content, index, one_or_zero);
}

fn content_calc<'a>(content: Vec<&'a str>, i: usize, one_or_zero: bool) -> Vec<&'a str> {
    let mut new_content: Vec<&str> = vec![];
    for element in content {
        if one_or_zero {
            if element.as_bytes()[i] == 49 {
                new_content.push(element);
            }
        } else {
            if element.as_bytes()[i] == 48 {
                new_content.push(element);
            }
        }
    }

    return new_content;
}

fn main() {
    let file = "data.txt";
    let data = fs::read_to_string(file).unwrap();

    let new_data = calculate(data.split("\r\n").collect::<Vec<&str>>(), 0);

    let new_data = calculate(new_data, 1);
    let new_data = calculate(new_data, 2);
    let new_data = calculate(new_data, 3);
    let new_data = calculate(new_data, 4);
    let new_data = calculate(new_data, 5);
    let new_data = calculate(new_data, 6);
    let new_data = calculate(new_data, 7);
    let new_data = calculate(new_data, 8);

    // Run if OGR
    let new_data = calculate(new_data, 9);
    let new_data = calculate(new_data, 10);
    let new_data = calculate(new_data, 11);

    println!("{:?}", new_data);

    // OGR: 011101011011 = 1883
    // CO2: 111000100111 = 3623
    println!("Answer: {}", 1883 * 3623);
}
