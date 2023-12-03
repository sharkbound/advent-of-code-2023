use daytemplate::{Day, DayPart};
use nom;
use nom::character::complete::{alphanumeric1, line_ending};
use nom::{IResult};

pub struct Day1Part1;

impl Day1Part1 {
    pub fn new() -> Self {
        Day1Part1 {}
    }

    fn nom_get_rows<'a>(&self, input: &'a str) -> IResult<&'a str, Vec<&'a str>> {
        Ok(nom::multi::separated_list0(line_ending, alphanumeric1)(input)?)
    }

    fn get_numbers(&self, input: &str) -> Vec<u32> {
        input.chars().filter_map(|c| c.to_digit(10)).collect()
    }
}

impl Day for Day1Part1 {
    type ParseOutput = Vec<u32>;

    fn part() -> DayPart {
        DayPart::ONE
    }

    fn day() -> i32 {
        1
    }

    fn parse(&self, input: &str) -> Self::ParseOutput {
        let lines = self.nom_get_rows(input).expect("Could not parse input").1;
        let mut numbers = Vec::new();
        for line in lines {
            let line_numbers = self.get_numbers(line);
            let (first, last) = (*line_numbers.first().unwrap(), *line_numbers.last().unwrap());
            numbers.push(format!("{}{}", first, last).parse::<u32>().unwrap());
        }
        numbers
    }

    fn solve(&self) {
        let input = self.input();
        let parsed = self.parse(&input);
        println!("Day 1 Part 1: {:?}", parsed.iter().sum::<u32>());
    }
}
