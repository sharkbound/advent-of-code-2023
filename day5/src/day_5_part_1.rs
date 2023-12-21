#![allow(dead_code, unused_variables)]

use nom::bytes::complete::{tag, tag_no_case, take_until};
use nom::character::complete::{digit1, multispace1, space1};
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::{many0, separated_list0};
use nom::sequence::{tuple};
use daytemplate::{Day, DayPart};
use rustutils::nom_helpers::consume_empty_space;

pub struct Day5Part1;

impl Day5Part1 {
    pub fn new() -> Self {
        Day5Part1 {}
    }
}

impl Day for Day5Part1 {
    type ParseOutput = Vec<Group>;

    fn part() -> DayPart {
        DayPart::ONE
    }

    fn day() -> i32 {
        5
    }

    fn parse(&self, input: &str) -> Self::ParseOutput {
        nom_parse_input(input).expect("").1
    }

    fn solve(&self) {
        let input = self.sample("part_1").replace("\r\n", "\n");
        let parsed = self.parse(&input);
        dbg!(&parsed);
    }
}

#[derive(Debug)]
pub struct Group {
    heading: String,
    numbers: Vec<GroupRow>,
}

fn nom_parse_input(input: &str) -> IResult<&str, Vec<Group>> {
    let (input, seeds) = nom_parse_seeds(input)?;
    let (input, _) = consume_empty_space(input)?;
    let (input, groups) = separated_list0(tag("\n\n"), nom_parse_grouping)(input)?;
    Ok((input, groups))
}

fn nom_parse_grouping(input: &str) -> IResult<&str, Group> {
    let (input, _) = consume_empty_space(input)?;
    let (input, heading) = take_until(" ")(input)?;
    let (input, _) = take_until("\n")(input)?;
    let (input, _) = consume_empty_space(input)?;
    let (input, numbers) = nom_parse_digit_triplet(input)?;

    Ok((input, Group { heading: heading.to_string(), numbers }))
}

fn nom_parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = tag_no_case("seeds: ")(input)?;
    let (input, seeds) = separated_list0(multispace1, map_res(digit1, |s: &str| s.parse::<u64>()))(input)?;
    Ok((input, seeds))
}
#[derive(Debug, Copy, Clone)]
pub struct GroupRow {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl GroupRow {
    pub fn new(destination_range_start: u64, source_range_start: u64, range_length: u64) -> Self {
        Self {
            destination_range_start,
            source_range_start,
            range_length,
        }
    }
}
fn nom_parse_digit_triplet(input: &str) -> IResult<&str, Vec<GroupRow>> {
    let (input, matches) = many0(tuple((consume_empty_space, digit1, space1, digit1, space1, digit1)))(input)?;
    let mut numbers = Vec::new();
    for (_, d1, _, d2, _, d3) in matches {
        numbers.push(GroupRow::new(d1.parse::<u64>().unwrap(), d2.parse::<u64>().unwrap(), d3.parse::<u64>().unwrap()));
    }
    Ok((input, numbers))
}