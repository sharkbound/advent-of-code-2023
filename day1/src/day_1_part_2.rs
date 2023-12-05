use daytemplate::{Day, DayPart};
use nom;
use nom::character::complete::{alphanumeric1, line_ending, one_of};
use nom::{IResult};
use nom::branch::alt;
use nom::bytes::complete::{tag_no_case};
use nom::combinator::map_res;

pub struct Day1Part2;

impl Day1Part2 {
    pub fn new() -> Self {
        Self {}
    }

    fn nom_get_rows<'a>(&self, input: &'a str) -> IResult<&'a str, Vec<&'a str>> {
        Ok(nom::multi::separated_list0(line_ending, alphanumeric1)(input)?)
    }

    fn _nom_match_single_number_string(input: &str) -> IResult<&str, u32> {
        Ok(map_res(
            alt(
                (tag_no_case("one"), tag_no_case("two"), tag_no_case("three"), tag_no_case("four"), tag_no_case("five"),
                tag_no_case("six"), tag_no_case("seven"), tag_no_case("eight"), tag_no_case("nine"))
            ),
            |s: &str| Result::<u32, nom::error::ErrorKind>::Ok(match s {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => unreachable!("Unexpected number string: {:?}", s),
            }))(input)?)
    }

    fn _nom_match_numeric_number(input: &str) -> IResult<&str, u32> {
        Ok(map_res(one_of("123456789"), |c: char| c.to_digit(10).ok_or_else(|| nom::error::ErrorKind::Digit))(input)?)
    }

    fn nom_get_numbers(&self, input: &str) -> Vec<u32> {
        let mut numbers = Vec::new();
        let mut current = input;

        while !current.is_empty() {
            // Try to check if it's a number in string format
            if let Ok((_, number)) = Self::_nom_match_single_number_string(current) {
                numbers.push(number);
                current = &current[1..];
                continue;
            }

            // Try to parse as a number
            if let Ok((_, number)) = Self::_nom_match_numeric_number(current) {
                numbers.push(number);
                current = &current[1..];
                continue;
            }
            current = &current[1..];
        }
        numbers
    }
}

impl Day for Day1Part2 {
    type ParseOutput = Vec<u32>;

    fn part() -> DayPart {
        DayPart::TWO
    }

    fn day() -> i32 {
        1
    }

    fn parse(&self, input: &str) -> Self::ParseOutput {
        let lines = self.nom_get_rows(input).expect("Could not parse input").1;
        let mut numbers = Vec::new();
        for line in lines {
            let line_numbers = self.nom_get_numbers(line);
            let (first, last) = (*line_numbers.first().unwrap(), *line_numbers.last().unwrap());
            println!("{:?} | {:?} | {:?}", line, (first, last), line_numbers);
            numbers.push(format!("{}{}", first, last).parse::<u32>().unwrap());
        }
        numbers
    }

    fn solve(&self) {
        // let input = self.sample("part_2");
        let input = self.input();
        let parsed = self.parse(&input);
        println!("Day 1 Part 2: {:?}", parsed.iter().sum::<u32>());
    }
}
