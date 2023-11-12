use std::fs;
use std::{env, process::exit};

fn read_file(filepath: &String) -> Vec<String> {
    let content = fs::read_to_string(filepath).expect("file was not able to be open");
    let mut items: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
    items.pop();
    items
}

fn convert_vec_into_number(vec: &Vec<u32>) -> u32 {
    let gamma_rate: u32 =
        u32::from_str_radix(&vec.iter().map(|x| x.to_string()).collect::<String>(), 2)
            .expect("error");
    gamma_rate
}

fn parse_data_into_vec(data: &Vec<String>) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();
    for string in data {
        let number = string
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        numbers.push(convert_vec_into_number(&number));
    }
    numbers
}

fn count_digits(data: &Vec<u32>, position: usize, digit: u32) -> u32 {
    let mut count: u32 = 0;
    for i in 0..data.len() {
        let number = data[i];
        let number = number >> position;
        let number = number & 1;
        if number == digit {
            count += 1;
        }
    }
    count
}

fn count_coincidences(data: &Vec<u32>, value: u32, len: usize) -> Vec<u32> {
    let mut count_ones: Vec<u32> = Vec::new();
    for i in 0..len {
        count_ones.push(count_digits(data, i, value));
    }
    count_ones.reverse();
    count_ones
}
fn compare_vecs(ones: &Vec<u32>, zeros: &Vec<u32>) -> Vec<u32> {
    let mut new_vec: Vec<u32> = Vec::new();
    for i in 0..ones.len() {
        if ones[i] > zeros[i] {
            new_vec.push(1);
        } else {
            new_vec.push(0);
        }
    }
    new_vec
}

fn invert_vec(vec: &Vec<u32>) -> Vec<u32> {
    let mut new_vec: Vec<u32> = Vec::new();
    for i in 0..vec.len() {
        if vec[i] == 1 {
            new_vec.push(0);
        } else {
            new_vec.push(1);
        }
    }
    new_vec
}
enum Rule {
    Most,
    Less,
}

fn parse_data(data: &Vec<String>) -> Vec<Vec<u32>> {
    let mut numbers: Vec<Vec<u32>> = Vec::new();
    for string in data {
        let number = string
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u32)
            .collect::<Vec<u32>>();
        numbers.push(number);
    }
    numbers
}

fn shrink_vec(data: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    if data.len() > 0 && data[0].len() == 1 {
        return data.clone();
    }
    let mut new_vec: Vec<Vec<u32>> = Vec::new();
    for vec in data {
        let mut new_vec2: Vec<u32> = Vec::new();
        for i in 1..vec.len() {
            new_vec2.push(vec[i]);
        }
        new_vec.push(new_vec2);
    }
    new_vec
}

fn filter_vec(data: &Vec<Vec<u32>>, rule: &Rule, index: usize) -> Vec<Vec<u32>> {
    let mut new_vec: Vec<Vec<u32>> = Vec::new();
    let n_elements = data.len();
    let mut n_ones = 0;
    for vec in data {
        if vec[index] == 1 {
            n_ones += 1;
        }
    }
    let n_zeros = n_elements - n_ones;
    for element in data {
        match rule {
            Rule::Most => {
                if element[index] == 1 && n_ones >= n_zeros {
                    new_vec.push(element.clone());
                } else if element[index] == 0 && n_zeros > n_ones {
                    new_vec.push(element.clone());
                }
            }
            Rule::Less => {
                if element[index] == 0 &&  n_zeros <= n_ones{
                    new_vec.push(element.clone());
                } else if element[index] == 1 &&  n_ones < n_zeros{
                    new_vec.push(element.clone());
                }
            }
        }
    }
    new_vec
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No filepath in args");
        exit(1);
    }
    let filepath = &args[1];
    let content: Vec<String> = read_file(filepath);
    let numbers = parse_data_into_vec(&content);
    let len_vec = content[0].len();

    let cout_ones = count_coincidences(&numbers, 1, len_vec);
    let cout_zeros = count_coincidences(&numbers, 0, len_vec);
    let gamma_rate = compare_vecs(&cout_ones, &cout_zeros);
    let epsilon_rate = invert_vec(&gamma_rate);
    let gamma_rate = convert_vec_into_number(&gamma_rate);
    let epsilon_rate = convert_vec_into_number(&epsilon_rate);


    println!("gamma_rate: {}", gamma_rate);
    println!("epsilon_rate: {}", epsilon_rate);
    println!("power consumption: {}", gamma_rate * epsilon_rate);

    let mut data = parse_data(&content);
    let rule = Rule::Most;
    for i in 0..data[0].len() {
        data = filter_vec(&data, &rule, i);
        if data.len() == 1 {
            break;
        }
    }
    let oxigen = convert_vec_into_number(&data[0]);
    println!("oxigen: {}", oxigen);

    let rule = Rule::Less;
    let mut data = parse_data(&content);
    for i in 0..data[0].len() {
        data = filter_vec(&data, &rule, i);
        if data.len() == 1 {
            break;
        }
    }
    let co2 = convert_vec_into_number(&data[0]);
    println!("co2: {}", co2);
    println!("life support rating: {}", oxigen * co2);
}
