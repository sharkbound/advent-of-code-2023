use std::collections::HashMap;
use nom::bytes::complete::{tag};
use nom::character::complete::{digit1, line_ending, multispace0, multispace1};
use nom::combinator::{map, map_res};
use nom::IResult;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{separated_pair, tuple};
use daytemplate::{Day, DayPart};
use rustutils::collections::CollectToVec;
use rustutils::iterator_extensions::DerefItems;

pub struct Day4Part2 {}

impl Day4Part2 {
    pub fn new() -> Day4Part2 {
        Day4Part2 {}
    }
}

impl Day for Day4Part2 {
    type ParseOutput = Vec<Card>;

    fn part() -> DayPart {
        DayPart::TWO
    }

    fn day() -> i32 {
        4
    }

    fn parse(&self, input: &str) -> Self::ParseOutput {
        let result = nom_parse_input(input);
        result.expect("error parsing cards").1
    }

    fn solve(&self) {
        // let input = self.sample("part_1");
        let input = self.input();
        let parsed = self.parse(&input);
        let cards = parsed.iter().map(|c| (c.id, c)).collect::<HashMap<u32, &Card>>();
        let mut counts = HashMap::<u32, u32>::new();
        for card in parsed.iter() {
            let win_count = winning_card_count(card);
            *counts.entry(card.id).or_default() += 1;
            let copy_count = counts.get(&card.id).map(|c| *c).unwrap_or(1);
            for i in 1..=win_count {
                *counts.entry(card.id + i).or_default() += copy_count;
            }
        }

        println!("Day 4 Part 2: {}", counts.iter().map(|x| *x.1).sum::<u32>());
    }
}

fn winning_card_count(card: &Card) -> u32 {
    card.owned.iter().filter(|c| card.winning.contains(c)).count() as u32
}

fn double(val: u32) -> u32 {
    match val {
        0 => 1,
        1 => 2,
        x => x * 2,
    }
}

fn nom_parse_input(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, cards) = separated_list0(line_ending, nom_parse_line)(input)?;
    Ok((input, cards))
}

#[derive(Debug)]
pub struct Card {
    id: u32,
    winning: Vec<u32>,
    owned: Vec<u32>,
}

fn nom_parse_line(input: &str) -> IResult<&str, Card> {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    let (input, _) = tag("Card")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, id) = map_res(digit1, |x: &str| x.parse::<u32>())(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, (winning, owned)) = separated_pair(
        nom_parse_number_list,
        tuple((multispace0, tag("|"), multispace0)),
        nom_parse_number_list,
    )(input)?;
    Ok((input, Card { id, winning, owned }))
}

fn nom_parse_number_list(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = multispace0(input)?;
    let (input, numbers) = separated_list1(multispace1, digit1)(input)?;
    let numbers = numbers.iter().map(|s| s.parse::<u32>().unwrap()).collect_to_vec();
    Ok((input, numbers))
}