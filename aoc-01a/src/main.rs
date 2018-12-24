use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
  let x = frequency(0,readfile());
  println!("{}", x);
}

fn readfile() -> Vec<String>{
  let file = File::open("input.txt").unwrap();;
  let reader = io::BufReader::new(file);
  reader.lines().map(|l| l.expect("Could not parse line")).collect()
}

fn frequency(start:i32, input: Vec<String>) -> i32 {
  let mut result = start;
  for val in input.iter() {
    result += val.parse::<i32>().unwrap();
  } 
  return result
}

#[cfg(test)]
mod tests;
