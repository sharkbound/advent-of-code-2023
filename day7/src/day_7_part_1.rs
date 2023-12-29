use std::collections::HashMap;
use nom::character::complete::{alphanumeric1, digit1, line_ending};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use daytemplate::{Day, DayPart};
use rustutils::collections::CollectToVec;
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
    label: char,
    value: u32,
}

impl Card {
    fn new(char: char) -> Self {
        /*
        A, K, Q, J, T
        14 13 12 11 10
         */
        Self {
            label: char,
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

impl Hand {}

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



enum HandMatch {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandMatch {
    fn score(&self) -> u32 {
        match self {
            HandMatch::FiveOfAKind => 7,
            HandMatch::FourOfAKind => 6,
            HandMatch::FullHouse => 5,
            HandMatch::ThreeOfAKind => 4,
            HandMatch::TwoPair => 3,
            HandMatch::OnePair => 2,
            HandMatch::HighCard => 1,
        }
    }
}

struct CalculatedHandResult {
    hand_match: HandMatch,
    label: char,
}



/*
Five of a kind, where all five cards have the same label:
    AAAAA
Four of a kind, where four cards have the same label and one card has a different label:
    AAAA8
Full house, where three cards have the same label, and the remaining two cards share a different label:
    33322
Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand:
    TTT98
Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label:
    22334
One pair, where two cards share one label, and the other three cards have a different label from the pair and each other:
    234AA
High card, where all cards' labels are distinct:
    23456
*/

struct CondensedCard {
    card: Card,
    count: u32,
}
fn calc_hand_match(hand: &Hand) -> CalculatedHandResult {
    let label_to_count = hand.cards.iter().fold(HashMap::new(), |mut h, card| {
        h.entry(card.label).and_modify(|v| *v += 1).or_insert(1);
        h
    });
    let counts = label_to_count.iter().map(|(&label, &count)| CondensedCard {card: Card::new(label), count }).collect_to_vec();
    let find_by_count = |count: u32| counts.iter().filter(|c| c.count == count).next();
    if let Some(result) = find_by_count(5) {
        return CalculatedHandResult {
            hand_match: HandMatch::FiveOfAKind,
            label: result.card.label,
        };
    }
    // five of a kind
    if let Some(result) = find_by_count(4) {
        return CalculatedHandResult {
            hand_match: HandMatch::FourOfAKind,
            label: result.card.label,
        };
    }
    // full house
    if let (Some(first), Some(second)) = (find_by_count(3), find_by_count(2))  {
        return CalculatedHandResult {
            hand_match: HandMatch::FullHouse,
            label: first.card.label, // todo: need to store this differently i think
        }
    }
    // three of a kind
    if let (Some(first), Some(second), Some(third)) = (find_by_count(3), find_by_count(1), find_by_count(1)) {
        return CalculatedHandResult {
            hand_match: HandMatch::ThreeOfAKind,
            label: first.card.label,
        }
    }
    todo!()
}