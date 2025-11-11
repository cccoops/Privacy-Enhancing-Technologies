use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
// Function to parse a string into an integer, returns Result type
pub fn parse_integer(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(format!("Invalid integer: {}", s)),
    }
}

// File I/O

// Function to read all lines from a file, returns Result type with Vec<String>
pub fn read_file_lines<P: AsRef<Path>>(file_path: P) -> Result<Vec<String>, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<String>, io::Error>>()?;
    Ok(lines)
}

// // Function to write a string to a file, returns Result type
// pub fn write_file<P: AsRef<Path>>(file_path: P, content: &str) -> Result<(), io::Error> {}

// // Combining Error Handling and File I/O

// // Function to read integers from file, compute their sum, and return the result
// pub fn read_and_sum_integers<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {}
