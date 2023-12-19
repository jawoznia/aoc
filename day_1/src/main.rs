use std::fs::read_to_string;

fn merge_first_and_last_digit(line: &str) -> Option<u32> {
    let first = line.chars().find(|c| c.is_ascii_digit())?;
    let last = line.chars().rev().find(|c| c.is_ascii_digit())?;
    // let num = format!("{}{}", first, last).parse::<u32>().ok();
    // println!("{:?}", num);
    // num
    None
}

fn main() -> Result<(), std::io::Error> {
    let filename = std::env::args().next().unwrap();
    let sum = read_to_string(filename)?
        .lines()
        .flat_map(merge_first_and_last_digit)
        .sum::<u32>();
    println!("{}", sum);

    Ok(())
}
