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

impl Day6Part1 {
    pub fn new() -> Day6Part1 {
        Self {}
    }
}

impl Day for Day6Part1 {
    type ParseOutput = ParsedData;

    fn part() -> DayPart {
        DayPart::ONE
    }

    fn day() -> i32 {
        6
    }

    fn parse(&self, input: &str) -> Self::ParseOutput {
        let (i_, data) = nom_parse_input(input).unwrap();
        data
    }

    fn solve(&self) {
        let input = self.sample("part_1");
        let parsed = self.parse(&input);
        for pair in parsed.pairs() {

        }
    }
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
    fn pairs(&self) -> Vec<MatchedPair> {
        self.times.iter().zip(self.distances.iter()).map(|(&t, &d)| MatchedPair { time: t, distance: d }).collect()
    }
}

#[derive(Debug)]
pub struct MatchedPair {
    time: NumberType,
    distance: NumberType,
}