use std::fs;
// use std::ops::IndexMut;
use inline_colorization::*;
use std::ops::{Index, IndexMut};
use std::{env, process::exit};

struct Matrix {
    data: Vec<u32>,
    rows: u32,
    cols: u32,
}

impl Matrix {
    fn new(rows: u32, cols: u32, value: u32) -> Matrix {
        let data: Vec<u32> = vec![value; (rows * cols) as usize];
        Matrix { data, rows, cols }
    }
    fn at(&self, i: u32, j: u32) -> u32 {
        let index = i * self.cols + j;
        self.data[index as usize]
    }
    fn new_from_str(str: &str) -> Matrix {
        let mut n_rows: u32 = 0;
        let mut n_cols: u32 = 0;
        let mut data: Vec<u32> = Vec::new();

        for line in str.lines() {
            n_rows += 1;
            let elems: Vec<u32> = line
                .split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            n_cols = elems.len() as u32;
            for elem in elems {
                data.push(elem);
            }
        }

        Matrix {
            data,
            rows: n_rows,
            cols: n_cols,
        }
    }

    fn win(&self) -> bool {
        // check by rows
        for i in 0..self.rows {
            let mut sum: u32 = 0;
            for j in 0..self.cols {
                sum += self.at(i, j);
            }
            if sum == 0 {
                return true;
            }
        }
        // check by cols
        for j in 0..self.cols {
            let mut sum: u32 = 0;
            for i in 0..self.rows {
                sum += self.at(i, j);
            }
            if sum == 0 {
                return true;
            }
        }
        false
    }
}

struct BingoCard {
    card: Matrix,
    checked_numbers: Matrix,
    last_checked_number: u32,
}

impl BingoCard {
    fn new_from_str(str: &str) -> BingoCard {
        let card = Matrix::new_from_str(str);
        let checked_numbers = Matrix::new(card.rows, card.cols, 1);
        BingoCard {
            card,
            checked_numbers,
            last_checked_number: 0,
        }
    }
    fn check_number(&mut self, number: u32) -> bool {
        self.last_checked_number = number;
        if let Some(index) = self.card.data.iter().position(|x| x == &number) {
            self.checked_numbers.data[index as usize] = 0;
        }
        self.checked_numbers.win()
    }

    fn compute_score(&self) -> u32 {
        let mut score: u32 = 0;
        for i in 0..self.checked_numbers.data.len() {
            if self.checked_numbers[i] != 0 {
                score += self.card[i];
            }
        }
        score * self.last_checked_number
    }
}

use std::fmt;

impl fmt::Display for BingoCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;
        for i in 0..self.checked_numbers.rows {
            for j in 0..self.checked_numbers.cols {
                if self.checked_numbers.at(i, j) == 0 {
                    write!(
                        f,
                        "{} , ",
                        format!("{color_red}{:2}{color_reset}", self.card.at(i, j))
                    )?;
                } else {
                    write!(f, "{:2} , ", self.card.at(i, j))?;
                }
            }
            write!(f, "\n")?;
        }
        std::fmt::Result::Ok(())
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n")?;
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{:2} , ", self.at(i, j))?;
            }
            write!(f, "\n")?;
        }
        std::fmt::Result::Ok(())
    }
}

impl Index<usize> for Matrix {
    type Output = u32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No filepath in args");
        exit(1);
    }
    let filepath = &args[1];
    let content = fs::read_to_string(filepath).expect("file was not able to be open");
    let lines: Vec<&str> = content.split("\n\n").collect();
    let (numbers, bingos) = lines.split_first().unwrap();
    let numbers: Vec<u32> = numbers
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut matrices: Vec<BingoCard> = Vec::new();
    for bingo in bingos {
        let matrix = BingoCard::new_from_str(bingo.trim());
        matrices.push(matrix);
    }

    let mut n_winners = 0;
    let max_winners = matrices.len();
    'outer: for number in &numbers {
        for matrix in &mut matrices {
            if !matrix.checked_numbers.win() && matrix.check_number(*number) {
                n_winners += 1;
                if n_winners == 1 {
                    println!("Part1 Score : {}", matrix.compute_score());
                }
                if n_winners == max_winners {
                    // println!("final number : {}", number);
                    println!("Part2 Score : {}", matrix.compute_score());
                    break 'outer;
                }
            }
        }
    }
}
