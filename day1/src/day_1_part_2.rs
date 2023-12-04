use daytemplate::{Day, DayPart};
use nom;
use nom::character::complete::{alphanumeric1, line_ending};
use nom::{IResult};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};

pub struct Day1Part2;

impl Day1Part2 {
    pub fn new() -> Self {
        Self {}
    }

    fn nom_get_rows<'a>(&self, input: &'a str) -> IResult<&'a str, Vec<&'a str>> {
        Ok(nom::multi::separated_list0(line_ending, alphanumeric1)(input)?)
    }

    fn _nom_match_single_number(input: &str) -> IResult<&str, &str> {
        Ok(alt((
                   tag("one"), tag("two"), tag("three"), tag("four"), tag("five"),
                   tag("six"), tag("seven"), tag("eight"), tag("nine"),
                   take_while_m_n(1, 1, |c: char| c.is_digit(10))),
        )(input)?)
    }

    fn nom_get_numbers(&self, input: &str) -> Vec<u32> {
        let mut numbers = Vec::new();
        let mut current = input;

        while !current.is_empty() {
            match Self::_nom_match_single_number(current) {
                Ok((input, nom_match)) => {
                    current = input;
                    match nom_match {
                        "" => continue,
                        "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                            numbers.push(nom_match.parse::<u32>().unwrap());
                        }

                        string => {
                            numbers.push(match string {
                                "one" => 1,
                                "two" => 2,
                                "three" => 3,
                                "four" => 4,
                                "five" => 5,
                                "six" => 6,
                                "seven" => 7,
                                "eight" => 8,
                                "nine" => 9,
                                _ => unreachable!("Unexpected number string: {:?}", string),
                            })
                        }
                    }
                }

                Err(_) => {
                    current = &current[1..];
                }
            }
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
            // println!("LINE: {:?}, NUMBERS: {:?}, FIRST-LAST: {:?}", line, line_numbers, (first, last));
            numbers.push(format!("{}{}", first, last).parse::<u32>().unwrap());
        }
        numbers
    }

    fn solve(&self) {
        // let input = self.sample("part_2");
        let input = self.input();
        let parsed = self.parse(&input);

        println!("Day 1 Part 2: {:?}", parsed.iter().sum::<u32>()); // should be 54728
    }
}
