use std::fs;
// use std::ops::IndexMut;
use std::ops::{Index, IndexMut};
use std::{env, process::exit};

struct Matrix {
    data: Vec<u32>,
    rows: u32,
    cols: u32,
}

impl Matrix {
    fn new(rows: u32, cols: u32) -> Matrix {
        Matrix {
            data: Vec::with_capacity((rows * cols) as usize),
            rows,
            cols,
        }
    }
    fn at(&self, i: u32, j: u32) -> u32 {
        let index = i*self.cols + j;
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

    fn win(&self)->bool{
        // check by rows 
        for i in 0..self.rows{
            let mut sum:u32 = 0;
            for j in 0..self.cols{
                sum += self.at(i,j);
            }
            if sum == 0 {return  true;}
        }
        // check by cols 
        for j in 0..self.cols{
            let mut sum:u32 = 0;
            for i in 0..self.rows{
                sum += self.at(i,j);
            }
            if sum == 0 {return  true;}
        }
        false

    }
    fn check_number(&mut self, number:u32)->bool{
        if let Some(value) = self.data.iter().find(|x| **x == number).as_mut(){
            *value=&0_u32;
        }
        self.win()
    }
}

use std::fmt;

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

fn read_matrices(str: &str) -> Vec<Matrix> {
    Vec::new()
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

    let mut matrices : Vec<Matrix> = Vec::new();
    for bingo in bingos{
        let matrix = Matrix::new_from_str(bingo.trim());
        println!("matrix : {}", matrix);
        matrices.push(matrix);
    }

    for number in &numbers{
        println!("Matrix[0] {}\n,{}",matrices[0],number);
        for matrix in &mut matrices{
            if matrix.check_number(*number){
                println!("Matrix {}\n, won with number {}",matrix,number);
            }
        }
    }
    
    // println!("File content: {}", content);
}
