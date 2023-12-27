use nom::{IResult, Parser};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, multispace0, multispace1};
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::sequence::tuple;

use daytemplate::{Day, DayPart};
use rustutils::nom_helpers::consume_empty_space;

type NumberType = i64;

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
        let distance = lines[1][lines[1].find(':').unwrap() + 1..].replace(" ", "").parse::<NumberType>().unwrap();
        RaceRecord { time, distance }
    }

    fn solve(&self) {
        // let input = self.sample("part_1");
        let input = self.input();
        let parsed = self.parse(&input);
        let center = parsed.time / 2;

        let mut ways_to_win = 0;
        for hold_time in DirectionalNumberRange::new(center - 1, -1) {
            if hold_button(&parsed, hold_time) > parsed.distance {
                ways_to_win += 1;
            } else { break; }
        }
        for hold_time in DirectionalNumberRange::new(center + 1, 1) {
            if hold_button(&parsed, hold_time) > parsed.distance {
                ways_to_win += 1;
            } else { break; }
        }
        if hold_button(&parsed, center) > parsed.distance {
            ways_to_win += 1;
        }
        println!("Day 6 Part 2: {:?}", ways_to_win);
    }
}

fn hold_button(record: &RaceRecord, hold_time: NumberType) -> NumberType {
    if hold_time >= record.time {
        return 0;
    }
    (record.time - hold_time) * hold_time
}


struct DirectionalNumberRange {
    current: NumberType,
    started: bool,
    direction: NumberType,
}

impl DirectionalNumberRange {
    fn new(current: NumberType, direction: NumberType) -> DirectionalNumberRange {
        Self {
            current,
            started: false,
            direction,
        }
    }
}

impl Iterator for DirectionalNumberRange {
    type Item = NumberType;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            Some(self.current)
        } else {
            let ret = self.current + self.direction;
            self.current = ret;
            Some(ret)
        }
    }
}

#[derive(Debug)]
pub struct RaceRecord {
    time: NumberType,
    distance: NumberType,
}