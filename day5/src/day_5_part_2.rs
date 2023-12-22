#![allow(dead_code, unused_variables)]

use std::ops::{Range};
use nom::bytes::complete::{tag, tag_no_case, take_until};
use nom::character::complete::{digit1, multispace1, space1};
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::{many0, separated_list0};
use nom::sequence::{tuple};
use daytemplate::{Day, DayPart};
use rustutils::collections::CollectToVec;
use rustutils::nom_helpers::consume_empty_space;

pub struct Day5Part2;

impl Day5Part2 {
    pub fn new() -> Self {
        Day5Part2 {}
    }
}

impl Day for Day5Part2 {
    type ParseOutput = (Vec<Group>, Vec<u64>);

    fn part() -> DayPart {
        DayPart::TWO
    }

    fn day() -> i32 {
        5
    }

    fn parse(&self, input: &str) -> Self::ParseOutput {
        nom_parse_input(input).expect("").1
    }

    fn solve(&self) {
        // let input = self.sample("part_1").replace("\r\n", "\n");
        let input = self.input().replace("\r\n", "\n");
        let (groups, seeds) = self.parse(&input);
        let seed_ranges = seeds
            .chunks_exact(2)
            .map(|range| range[0]..(range[0] + range[1]))
            .collect_to_vec();

        let groups_assigned = GroupAssignments {
            seed_to_soil: &groups[0],
            soil_to_fertilizer: &groups[1],
            fertilizer_to_water: &groups[2],
            water_to_light: &groups[3],
            light_to_temperature: &groups[4],
            temperature_to_humidity: &groups[5],
            humidity_to_location: &groups[6],
        };

        let mut location = 0;
        loop {
            let mut seed = location;
            for group in groups_assigned.chain().iter().rev() {
                seed = group.reverse_search(seed);
            }
            if seed_ranges.iter().any(|x| x.contains(&seed)){
                break;
            }
            location += 1;
        }
        println!("Day 5 Part 2 Solution: {:?}", location);
    }
}

/*
// OLD ->
fn solve(&self) {
        let input = self.sample("part_1").replace("\r\n", "\n");
        // let input = self.input().replace("\r\n", "\n");
        let (groups, seeds) = self.parse(&input);
        let seed_ranges = seeds
            .chunks_exact(2)
            .map(|range| range[0]..(range[0] + range[1]))
            .collect_to_vec();

        let groups_assigned = GroupAssignments {
            seed_to_soil: &groups[0],
            soil_to_fertilizer: &groups[1],
            fertilizer_to_water: &groups[2],
            water_to_light: &groups[3],
            light_to_temperature: &groups[4],
            temperature_to_humidity: &groups[5],
            humidity_to_location: &groups[6],
        };

        let mut lowest_location = u64::MAX;
        for range in seed_ranges {
            for j in range {
                lowest_location = lowest_location.min(resolve_location_number(&groups_assigned, j));
            }
        }
        println!("Day 5 Part 2 Solution: {:?}", lowest_location);
    }
 */

fn resolve_location_number(groups: &GroupAssignments, initial_seed_number: u64) -> u64 {
    let mut source = initial_seed_number;
    let chain = groups.chain();
    for i in 0..7 {
        let group = chain[i];
        source = match group.rows.iter().find(|r| r.source_contains(source)) {
            None => source,
            Some(r) => r.destination_from_source(source),
        }
    }
    source
}
/*
        seed-to-soil ->
        soil-to-fertilizer ->
        fertilizer-to-water ->
        water-to-light ->
        light-to-temperature ->
        temperature-to-humidity ->
        humidity-to-location.
        */
struct GroupAssignments<'a> {
    seed_to_soil: &'a Group,
    soil_to_fertilizer: &'a Group,
    fertilizer_to_water: &'a Group,
    water_to_light: &'a Group,
    light_to_temperature: &'a Group,
    temperature_to_humidity: &'a Group,
    humidity_to_location: &'a Group,
}

impl<'a> GroupAssignments<'a> {
    fn chain(&self) -> [&'a Group; 7] {
        [
            self.seed_to_soil,
            self.soil_to_fertilizer,
            self.fertilizer_to_water,
            self.water_to_light,
            self.light_to_temperature,
            self.temperature_to_humidity,
            self.humidity_to_location,
        ]
    }
}

#[derive(Debug)]
pub struct Group {
    heading: String,
    rows: Vec<GroupRow>,
}

impl Group {
    fn reverse_search(&self, val: u64) -> u64 {
        for row in &self.rows {
            if row.destination_contains(val) {
                return row.source_range_start + (val - row.destination_range_start);
            }
        }
        val
    }
}

fn nom_parse_input(input: &str) -> IResult<&str, (Vec<Group>, Vec<u64>)> {
    let (input, seeds) = nom_parse_seeds(input)?;
    let (input, _) = consume_empty_space(input)?;
    let (input, groups) = separated_list0(tag("\n\n"), nom_parse_grouping)(input)?;
    Ok((input, (groups, seeds)))
}

fn nom_parse_grouping(input: &str) -> IResult<&str, Group> {
    let (input, _) = consume_empty_space(input)?;
    let (input, heading) = take_until(" ")(input)?;
    let (input, _) = take_until("\n")(input)?;
    let (input, _) = consume_empty_space(input)?;
    let (input, numbers) = nom_parse_digit_triplet(input)?;

    Ok((input, Group { heading: heading.to_string(), rows: numbers }))
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

    fn destination_from_source(&self, source: u64) -> u64 {
        if !self.source_contains(source) {
            return source; // unmapped sources directly translations to itself
        }
        // new source = destination + the offset from the source start
        self.destination_range_start + (source - self.source_range_start)
    }

    fn destination_range(&self) -> Range<u64> {
        self.destination_range_start..(self.destination_range_start + self.range_length)
    }

    fn source_range(&self) -> Range<u64> {
        self.source_range_start..(self.source_range_start + self.range_length)
    }

    fn source_contains(&self, number: u64) -> bool {
        self.source_range().contains(&number)
    }

    fn destination_contains(&self, number: u64) -> bool {
        self.destination_range().contains(&number)
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