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

fn first_solution(filepath: &str) -> Result<u32, std::io::Error> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .flat_map(|line| find_number(&line.unwrap()))
        .sum::<u32>())
}

fn word_to_digit(word: &str) -> &str {
    DIGITS[..10]
        .iter()
        .enumerate()
        .find(|(_, digit)| **digit == word)
        .map(|(i, _)| DIGITS[i + 10])
        .unwrap_or_else(|| word)
}

fn find_number_two(line: String) -> Option<u32> {
    let first =
        (0..line.len()).find_map(|i| DIGITS.iter().find(|digit| line[i..].starts_with(*digit)))?;
    let last = (0..line.len())
        .rev()
        .find_map(|i| DIGITS.iter().find(|digit| line[i..].starts_with(*digit)))?;

    let first = word_to_digit(first);
    let last = word_to_digit(last);

    format!("{}{}", first, last).parse::<u32>().ok()
}

fn solution_two(filepath: &str) -> Result<u32, std::io::Error> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .flatten()
        .flat_map(find_number_two)
        .sum::<u32>())
}

fn main() -> Result<(), std::io::Error> {
    let filepath = std::env::args().nth(1).unwrap();
    let sum = first_solution(&filepath)?;
    println!("First sum: {}", sum);
    let sum = solution_two(&filepath)?;
    println!("Second sum: {}", sum);

    Ok(())
}
