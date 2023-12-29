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
        for hand in &parsed {
            println!("{:?}", process_hand(hand));
        }
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


#[derive(Debug)]
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

#[derive(Debug)]
struct CalculatedHandResult {
    hand_match: HandMatch,
    labels: Vec<char>,
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

fn process_hand(hand: &Hand) -> CalculatedHandResult {
    let label_to_count = hand.cards.iter().fold(HashMap::new(), |mut h, card| {
        h.entry(card.label).and_modify(|v| *v += 1).or_insert(1);
        h
    });
    // let counts = label_to_count.iter().map(|(&label, &count)| CondensedCard { card: Card::new(label), count }).collect_to_vec();
    // let find_by_count = |count: u32| counts.iter().filter(|c| c.count == count).next();
    // five of a kind
    if let Some(card) = find_card_by_count(&hand, 5, &[]) {
        return CalculatedHandResult {
            hand_match: HandMatch::FiveOfAKind,
            labels: vec![card.card.label],
        };
    }
    // four of a kind
    if let Some(card) = find_card_by_count(&hand, 4, &[]) {
        return CalculatedHandResult {
            hand_match: HandMatch::FourOfAKind,
            labels: vec![card.card.label],
        };
    }
    // full house
    if let Some(matches) = find_many_cards_by_counts(&hand, &[3, 2]) {
        return CalculatedHandResult {
            hand_match: HandMatch::FullHouse,
            labels: matches.iter().map(|m| m.card.label).collect(),
        };
    }
    // three of a kind
    if let Some(card) = find_card_by_count(&hand, 3, &[]) {
        return CalculatedHandResult {
            hand_match: HandMatch::ThreeOfAKind,
            labels: vec![card.card.label],
        };
    }
    // two pair
    if let Some(matches) = find_many_cards_by_counts(&hand, &[2, 2]) {
        return CalculatedHandResult {
            hand_match: HandMatch::TwoPair,
            labels: matches.iter().map(|m| m.card.label).collect(),
        };
    }
    // one pair
    if let Some(card) = find_card_by_count(&hand, 2, &[]) {
        return CalculatedHandResult {
            hand_match: HandMatch::OnePair,
            labels: vec![card.card.label],
        };
    }
    // high card
    if let Some(card) = find_many_cards_by_counts(&hand, &[1, 1, 1, 1, 1]) {
        return CalculatedHandResult {
            hand_match: HandMatch::HighCard,
            labels: card.iter().map(|m| m.card.label).collect(),
        };
    }

    unreachable!("This should never be reached; if this is reached, please panic calmly and exit in a orderly fashion. However, if the issue is \
    that you are stuck in vim, don't bother asking for help. You are forever stuck in vim.")
}

fn find_card_by_count(hand: &Hand, expected_count: u32, excluded: &[char]) -> Option<CondensedCard> {
    for card in &hand.cards {
        if excluded.contains(&card.label) {
            continue;
        }
        let count = (&hand.cards).iter().filter(|c| c.label == card.label).count();
        if count == expected_count as usize {
            return Some(CondensedCard { card: *card, count: count as u32 });
        }
    }
    None
}

fn find_many_cards_by_counts(hand: &Hand, expected_counts: &[u32]) -> Option<Vec<CondensedCard>> {
    let mut excluded = Vec::with_capacity(5);
    let mut out = Vec::with_capacity(expected_counts.len());
    for expected_count in expected_counts {
        let card = find_card_by_count(hand, *expected_count, &excluded);
        match card {
            None => return None,
            Some(val) => {
                excluded.push(val.card.label);
                out.push(val);
            }
        }
    }
    if out.len() == expected_counts.len() {
        return Some(out);
    }
    return None;
}