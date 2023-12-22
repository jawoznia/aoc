use std::fs::File;
use std::io::{BufRead, BufReader};

use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, char, digit1};
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Default, Debug, PartialEq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn game(input: &str) -> IResult<&str, &str> {
    let (input, _) = take_until(":")(input)?;
    tag(":")(input)
}

fn count(input: &str) -> IResult<&str, &str> {
    digit1(input.trim())
}

fn color(input: &str) -> IResult<&str, &str> {
    alpha1(input.trim())
}

fn balls_amount(input: &str) -> IResult<&str, (&str, &str)> {
    // Read number of balls
    let (input, count) = count(input)?;

    // Read color of balls
    let (input, color) = color(input)?;
    Ok((input, (color, count)))
}

/// Parses up to 3 sets of colored balls
fn single_set(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(char(','), balls_amount)(input)
}

fn balls(input: &str) -> IResult<&str, Color> {
    // Game X:
    let (input, _) = game(input)?;

    // Read number and color of balls
    let (input, balls) = separated_list1(char(';'), single_set)(input)?;
    println!("Line\n");
    balls.iter().for_each(|ball| println!("Found {:?}", ball));

    //
    Ok((input, Color::default()))
}

fn read_file() -> Result<BufReader<File>, std::io::Error> {
    let filepath = std::env::args().nth(1).unwrap();
    let file = File::open(filepath)?;
    Ok(BufReader::new(file))
}

fn main() -> Result<(), std::io::Error> {
    let lines: Vec<_> = read_file()?.lines().collect::<Result<_, _>>()?;
    let balls: Vec<_> = lines.iter().map(|line| balls(line)).collect();
    balls.iter().for_each(|ball| println!("{:?}", ball));

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn game() {
        assert_eq!(super::game("Game 1: 3 blue"), Ok((" 3 blue", ":")));
        assert_eq!(super::count("1354"), Ok(("", "1354")));
    }
}
