use std::fs::File;
use std::io::{BufRead, BufReader};

fn merge_first_and_last_digit(line: &str) -> Option<u32> {
    let first = line.chars().find(|c| c.is_ascii_digit())?;
    let last = line.chars().rev().find(|c| c.is_ascii_digit())?;
    format!("{}{}", first, last).parse::<u32>().ok()
}

fn main() -> Result<(), std::io::Error> {
    let filepath = std::env::args().nth(1).unwrap();
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let sum = reader
        .lines()
        .flat_map(|line| merge_first_and_last_digit(&line.unwrap()))
        .sum::<u32>();
    println!("{}", sum);

    Ok(())
}
