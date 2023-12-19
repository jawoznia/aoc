use std::fs::File;
use std::io::{BufRead, BufReader};

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

fn main() -> Result<(), std::io::Error> {
    let filepath = std::env::args().nth(1).unwrap();
    let sum = first_solution(filepath)?;
    println!("Sum: {}", sum);

    Ok(())
}
