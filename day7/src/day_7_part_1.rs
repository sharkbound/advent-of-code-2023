use nom::branch::alt;
use nom::character::complete::{alpha1, alphanumeric1, crlf, digit1, line_ending, newline};
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use daytemplate::{Day, DayPart};
use rustutils::nom_helpers::consume_empty_space;

pub struct Day7Part1 {}

impl Day for Day7Part1 {
    type ParseOutput = Vec<Hand>;

    fn part() -> DayPart {
        DayPart::ONE
    }

    fn day() -> i32 {
        7
    }

    fn parse(&self, input: &str) -> Self::ParseOutput {
        let nom_parsed = nom_parse(input);
        return nom_parsed.unwrap().1;
    }

    fn solve(&self) {
        let input = self.sample("part_1");
        let parsed = self.parse(&input);
        println!("{:?}", parsed);
    }
}

#[derive(Copy, Clone, Debug)]
struct Card {
    char: char,
    value: u32,
}

impl Card {
    fn new(char: char) -> Self {
        /*
        A, K, Q, J, T
        14 13 12 11 10
         */
        Self {
            char,
            value: match char {
                ch @ '2'..='9' => ch.to_digit(10).unwrap(),
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unreachable!("Invalid card char: {}", char),
            },
        }
    }
}

#[derive(Debug)]
pub struct Hand {
    cards: [Card; 5],
    bet: u32,
}

impl Hand {

}

fn nom_parse(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(line_ending, nom_parse_hand)(input)
}

fn nom_parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, (hand_chars, _, bet)) = tuple((alphanumeric1, consume_empty_space, digit1))(input)?;
    let chars = hand_chars.chars().collect::<Vec<char>>();
    Ok((input, Hand {
        cards: [
            Card::new(chars[0]),
            Card::new(chars[1]),
            Card::new(chars[2]),
            Card::new(chars[3]),
            Card::new(chars[4])
        ],
        bet: bet.parse().unwrap(),
    }))
}