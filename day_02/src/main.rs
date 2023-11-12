use std::env;
use std::fs;
use std::process::exit;

#[derive(Debug)]
enum Mode {
    Basic,
    Advanced,
}

#[derive(Debug)]
struct Submarine {
    horizontal_position: i32,
    depth: i32,
    aim: i32,
    mode: Mode,
}

impl Submarine {
    fn new(mode: Mode) -> Submarine {
        Submarine {
            horizontal_position: 0,
            depth: 0,
            aim: 0,
            mode,
        }
    }
    fn forward(&mut self, positions: i32) {
        match self.mode {
            Mode::Basic => 
                self.horizontal_position += positions,
            Mode::Advanced => {
                self.horizontal_position += positions;
            self.depth += self.aim * positions; 
            }
        };
    }
    fn down(&mut self, positions: i32) {
        match self.mode {
            Mode::Basic => 
                self.depth += positions,
            Mode::Advanced => 
                self.aim += positions,
            }
    }
    fn up(&mut self, positions: i32) {
        match self.mode {
            Mode::Basic => 
                self.depth -= positions,
            Mode::Advanced => 
                self.aim -= positions,
            }
    }
    fn apply_actions(&mut self, content: &String) {
        let splits: Vec<&str> = content.split('\n').collect();
        for sentence in splits {
            // println!("{}",sentence);
            let words: Vec<&str> = sentence.split_whitespace().collect();
            if words.len() != 2 {
                continue;
            }
            // println!("words{:?}",words);
            let action = words[0];
            let number: i32 = words[1].trim().parse().expect("Error parsing number");
            println!("action:{}, number:{}", action, number);

            match action {
                "forward" => self.forward(number),
                "up" => self.up(number),
                "down" => self.down(number),
                _ => println!("Unknown action"),
            }
        }
    }
    fn compute_final(&self) -> i32 {
        self.horizontal_position * self.depth
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Filename or mode not provided");
        exit(0);
    }

    let filename: &str = args[1].as_str();
    let content = fs::read_to_string(filename).expect("Error reading the file");
    let mode = match args[2].as_str().parse::<i32>().expect("Error parsing mode") {
        1 => Mode::Basic,
        2 => Mode::Advanced,
        _ => {
            println!("Unknown mode");
            exit(0);
        }
    };


    
    let mut submarine = Submarine::new(mode);
    submarine.apply_actions(&content);
    println!("final number:{}", submarine.compute_final());
}
