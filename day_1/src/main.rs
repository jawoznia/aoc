use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DIGITS: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2",
    "3", "4", "5", "6", "7", "8", "9",
];

fn find_number(line: &str) -> Option<u32> {
    let first = line.chars().find(|c| c.is_ascii_digit())?;
    let last = line.chars().rev().find(|c| c.is_ascii_digit())?;
    format!("{}{}", first, last).parse::<u32>().ok()
}

fn first_solution(filepath: String) -> Result<u32, std::io::Error> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .flat_map(|line| find_number(&line.unwrap()))
        .sum::<u32>())
}

fn find_number_two<'a>(line: String) -> Option<&'a str> {
    // println!("{}", line);
    (0..line.len())
        .find_map(|i| DIGITS.iter().find(|digit| line[i..].starts_with(*digit)))
        .copied()
}

fn solution_two(filepath: &str) -> Result<u32, std::io::Error> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    // for line in reader.lines() {
    //     println!("{:?}", line);
    // }
    let first: Vec<_> = reader.lines().flatten().flat_map(find_number_two).collect();
    for digit in first {
        println!("{}", digit);
    }
    // println!("{:?}", first);
    // .sum::<u32>());

    Ok(0)
}

fn main() -> Result<(), std::io::Error> {
    let filepath = std::env::args().nth(1).unwrap();
    // let sum = first_solution(filepath)?;
    solution_two(&filepath)?;
    // println!("Sum: {}", sum);

    Ok(())
}
