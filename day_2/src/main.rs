use std::fs::File;
use std::io::{BufRead, BufReader};
use std::u32::MAX;

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

impl Balls {
    pub fn new(red: u32, green: u32, blue: u32) -> Self {
        Balls { red, green, blue }
    }

    pub fn condition_1(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    pub fn max(self, other: Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    pub fn mul(self) -> u32 {
        self.red * self.green * self.blue
    }
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

fn balls(input: &str) -> IResult<&str, Vec<Balls>> {
    let (input, _) = game(input)?;
    separated_list1(char(';'), single_set)(input)
}

fn read_file() -> Result<BufReader<File>, std::io::Error> {
    let filepath = std::env::args().nth(1).unwrap();
    let file = File::open(filepath)?;
    Ok(BufReader::new(file))
}

fn solution_1(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|line| balls(line).unwrap())
        .enumerate()
        .filter(|(_, (_, balls))| balls.iter().all(Balls::condition_1))
        .map(|(i, _)| i + 1)
        .sum()
}

fn solution_2(lines: &[String]) -> u32 {
    lines
        .iter()
        .map(|line| balls(line).unwrap())
        .map(|(_, balls)| {
            let balls = balls
                .into_iter()
                .fold(Balls::default(), |acc, balls| acc.max(balls));
            println!("{:?}", balls);
            balls.mul()
        })
        .sum::<u32>()
}

fn main() -> Result<(), std::io::Error> {
    let lines: Vec<_> = read_file()?.lines().collect::<Result<_, _>>()?;
    let solution_1: usize = solution_1(&lines);
    println!("Solution 1: {}", solution_1);
    let solution_2: u32 = solution_2(&lines);
    println!("Solution 2: {}", solution_2);

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
