// use std::fmt;
use std::fs;
use std::{env, process::exit};

fn count_increasing_measures(measures: &Vec<i32>) -> i32 {
    let mut n_measures_increasing = 0;
    for i in 0..measures.len() - 1 {
        if measures[i + 1] > measures[i] {
            n_measures_increasing += 1;
        }
    }
    n_measures_increasing
}

fn count_increasing_measures_with_sliding_window(measures: &Vec<i32>, window_size: usize) -> i32 {
    let mut sliding_window_values: Vec<i32> = Vec::new();
    for i in 0..measures.len() - window_size + 1 {
        let mut val = 0;
        for j in 0..window_size {
            val = val + measures[i + j];
        }
        sliding_window_values.push(val);
    }

    count_increasing_measures(&sliding_window_values)
}

fn read_filepath(filepath: &String) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(0);
    // read file and push to numbers
    let content = fs::read_to_string(filepath).expect("file was not able to be open");
    let items: Vec<&str> = content.split("\n").collect();
    for item in items {
        numbers.push(item.parse::<i32>().unwrap())
    }
    numbers
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No filepath in args");
        exit(1);
    }
    let filepath = &args[1];
    let numbers = read_filepath(filepath);
    let n_measures_increasing = count_increasing_measures(&numbers);
    println!(
        "The number of times that measures increases are {}",
        n_measures_increasing
    );
    let window_size: usize = 3;

    let n_measures_increasing_sliding_window: i32 =
        count_increasing_measures_with_sliding_window(&numbers, window_size);
    println!(
        "The number of times that measures increases using sliding window of {} are {}",
        window_size, n_measures_increasing_sliding_window
    );
}
