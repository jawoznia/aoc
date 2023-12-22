use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Sum;
use std::ops::Add;

use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, char, digit1};
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Default, Debug, PartialEq)]
pub struct Balls {
    red: u32,
    green: u32,
    blue: u32,
}

impl From<Vec<(&str, &str)>> for Balls {
    fn from(input: Vec<(&str, &str)>) -> Self {
        let mut balls = Balls::default();
        input.iter().for_each(|ball| {
            let (color, count) = ball;
            match *color {
                "red" => balls.red += count.parse::<u32>().unwrap(),
                "green" => balls.green += count.parse::<u32>().unwrap(),
                "blue" => balls.blue += count.parse::<u32>().unwrap(),
                _ => panic!("Unknown color"),
            }
        });
        balls
    }
}

impl Sum for Balls {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |acc, item| acc + item)
    }
}

impl Add for Balls {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Balls {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
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
fn single_set(input: &str) -> IResult<&str, Balls> {
    separated_list1(char(','), balls_amount)(input).map(|(input, balls)| (input, From::from(balls)))
}

fn balls(input: &str) -> IResult<&str, Balls> {
    // Game X:
    let (input, _) = game(input)?;

    // Read number and color of balls
    let (input, balls) = separated_list1(char(';'), single_set)(input)?;
    let balls: Balls = balls.into_iter().sum();
    println!("Line\n");
    println!("{:?}", balls);

    // balls.iter().for_each(|ball| println!("Found {:?}", ball));

    //
    Ok((input, Balls::default()))
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
