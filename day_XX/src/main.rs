use std::fs;
use std::{env, process::exit};

fn read_file(filepath: &String) -> Vec<String> {
    let content = fs::read_to_string(filepath).expect("file was not able to be open");
    let mut items: Vec<String> = content.split("\n").map(|s| s.to_string()).collect();
    items.pop();
    items
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No filepath in args");
        exit(1);
    }
    let filepath = &args[1];
    let content:Vec<String>= read_file(filepath);
    println!("{:?}", content);


}
