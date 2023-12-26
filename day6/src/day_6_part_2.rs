use nom::{IResult, Parser};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, multispace0, multispace1};
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::sequence::tuple;

use daytemplate::{Day, DayPart};
use rustutils::nom_helpers::consume_empty_space;

type NumberType = u64;

pub struct Day6Part2 {}

impl Day for Day6Part2 {
    type ParseOutput = RaceRecord;

    fn part() -> DayPart {
        DayPart::TWO
    }

    fn day() -> i32 {
        6
    }

    fn parse(&self, input: &str) -> Self::ParseOutput {
        let lines = input.lines().collect::<Vec<_>>();
        let time = lines[0][lines[0].find(':').unwrap() + 1..].replace(" ", "").parse::<NumberType>().unwrap();
        let distance = lines[1][lines[1].find(':').unwrap() + 1..].replace(" " , "").parse::<NumberType>().unwrap();
        RaceRecord { time, distance }
    }

    fn solve(&self) {
        // let input = self.sample("part_1");
        let input = self.input();
        let parsed = self.parse(&input);
        println!("Day 6 Part 2: {:?}", parsed);
    }
}

fn hold_button(record: &RaceRecord, hold_time: NumberType) -> NumberType {
    if hold_time >= record.time {
        return 0;
    }
    (record.time - hold_time) * hold_time
}

#[derive(Debug)]
pub struct RaceRecord {
    time: NumberType,
    distance: NumberType,
}