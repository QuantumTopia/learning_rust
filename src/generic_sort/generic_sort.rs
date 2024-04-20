#![allow(unused_imports)]
use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::utils::print_utils::mod_print as mod_print;

pub fn run_generic_sort() {
    welcome_message();

    match read_file_into_vector() {
        Ok(mut integers) => {
            print_vector(&mut integers);
            print_sorted_vector(&mut integers);
        }
        Err(err) => eprintln!("Error reading file: {:?}", err),
    }
}

fn welcome_message() -> () {
    mod_print("Welcome to the Generic sort machine!");
    mod_print("This tool sorts items found in the input/input.txt file");
    mod_print("");
}

fn read_file_into_vector() -> Result<Vec<i32>, io::Error> {
    let filepath = "src/input/input.txt";
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut integers = Vec::new();

    for line in reader.lines() {
        // Split the line by whitespace and parse each element into an integer
        let line = line?;
        for num_str in line.split_whitespace() {
            if let Ok(num) = num_str.parse::<i32>() {
                integers.push(num);
            } else {
                // Handle parsing error if needed
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Failed to parse integer",
                ));
            }
        }
    }

    Ok(integers)
}

fn print_vector(integers: &mut Vec<i32>) -> () {
    mod_print(format!("Your file input is: {:?}", integers).as_str());
}

fn print_sorted_vector(integers: &mut Vec<i32>) -> () {
    integers.sort();
    mod_print(format!("Your contents sorted: {:?}", integers).as_str());
} 