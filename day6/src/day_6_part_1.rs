use nom::{IResult, Parser};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, multispace0, multispace1};
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::sequence::tuple;

use daytemplate::{Day, DayPart};
use rustutils::nom_helpers::consume_empty_space;

type NumberType = u32;

pub struct Day6Part1 {}

impl Day for Day6Part1 {
    type ParseOutput = ParsedData;

    fn part() -> DayPart {
        DayPart::ONE
    }

    fn day() -> i32 {
        6
    }

    fn parse(&self, input: &str) -> Self::ParseOutput {
        let (_, data) = nom_parse_input(input).unwrap();
        data
    }

    fn solve(&self) {
        // let input = self.sample("part_1");
        let input = self.input();
        let parsed = self.parse(&input);
        let mut total = 1;
        for record in parsed.records() {
            total *= (1..=record.time).filter(|t| hold_button(&record, *t) > record.distance).count();
        }
        println!("Day 6 Part 1: {}", total);
    }
}

fn hold_button(record: &RaceRecord, hold_time: u32) -> u32 {
    if hold_time >= record.time {
        return 0;
    }
    (record.time - hold_time) * hold_time
}


fn nom_parse_input(input: &str) -> IResult<&str, ParsedData> {
    let (input, times) = nom_parse_row(input)?;
    let (input, distances) = nom_parse_row(input)?;
    Ok((input, ParsedData { times, distances }))
}

fn nom_parse_row(input: &str) -> IResult<&str, Vec<NumberType>> {
    let (input, (_, _, _, numbers, _)) = tuple((
        alpha1,
        tag(":"),
        multispace0,
        separated_list0(multispace1, map_res(digit1, |x: &str| x.parse::<NumberType>())),
        consume_empty_space,
    ))(input)?;
    Ok((input, numbers))
}

#[derive(Debug)]
pub struct ParsedData {
    times: Vec<NumberType>,
    distances: Vec<NumberType>,
}

impl ParsedData {
    fn records(&self) -> Vec<RaceRecord> {
        self.times.iter().zip(self.distances.iter()).map(|(&t, &d)| RaceRecord { time: t, distance: d }).collect()
    }
}

#[derive(Debug)]
pub struct RaceRecord {
    time: NumberType,
    distance: NumberType,
}