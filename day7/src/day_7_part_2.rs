#[allow(unused_variables, dead_code)]
use nom::character::complete::{alphanumeric1, digit1, line_ending};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use daytemplate::{Day, DayPart};
use rustutils::{join_to_string};
use rustutils::nom_helpers::consume_empty_space;

pub struct Day7Part2 {}

impl Day for Day7Part2 {
    type ParseOutput = Vec<Hand>;

    fn part() -> DayPart {
        DayPart::TWO
    }

    fn day() -> i32 {
        7
    }

    fn parse(&self, input: &str) -> Self::ParseOutput {
        let nom_parsed = nom_parse(input);
        return nom_parsed.unwrap().1;
    }


    fn solve(&self) {
        // let input = self.sample("part_1");
        let input = self.input();
        let mut parsed = self.parse(&input);

        parsed.sort_by_key(cmp_calculated_hand_result);

        let mut score = 0_u64;
        for (hand, multiplier) in parsed.iter().zip((1..=parsed.len()).rev()) {
            score += hand.bet as u64 * multiplier as u64;
            // println!("{} | {}: {}x", hand, hand.hand.bet, multiplier);
        }

        println!("Day 7 Part 2: {}", score);
    }
}

fn cmp_calculated_hand_result(hand: &Hand) -> [i32; 6] {
    let cmp_card_val = |hand: &Hand, idx: usize| -(hand.cards[idx].value as i32);
    [
        -(hand.upgraded().match_kind().score() as i32),
        cmp_card_val(hand, 0),
        cmp_card_val(hand, 1),
        cmp_card_val(hand, 2),
        cmp_card_val(hand, 3),
        cmp_card_val(hand, 4),
    ]
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct CondensedCard {
    card: Card,
    count: u32,
}

impl CondensedCard {
    #[allow(dead_code)]
    fn new(card: Card, count: u32) -> CondensedCard {
        CondensedCard { card, count }
    }

    fn label(&self) -> char {
        self.card.label
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Card {
    label: char,
    value: u32,
}

impl Card {
    fn new(char: char) -> Self {
        /*
        A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J.
        13 12 11 10 9  8  7  6  5  4  3  2  1
         */
        Self {
            label: char,
            value: match char {
                'J' => 1,
                ch @ '2'..='9' => ch.to_digit(10).unwrap(),
                'T' => 10,
                'Q' => 11,
                'K' => 12,
                'A' => 13,
                _ => unreachable!("Invalid card char: {}", char),
            },
        }
    }

    #[allow(dead_code)]
    fn empty() -> Self {
        Self {
            label: '2',
            value: 0,
        }
    }

    fn is_joker(&self) -> bool {
        self.label == 'J'
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Hand {
    cards: [Card; 5],
    bet: u32,
}

impl Hand {
    #[allow(dead_code)]
    fn from_cards_and_bet(cards: [Card; 5], bet: u32) -> Self {
        Self {
            cards,
            bet,
        }
    }
    fn find_most_common_card(&self) -> CondensedCard {
        let ignored = &['J'];
        if let Some(card) = self.find_card_by_count(5, ignored) {
            return card;
        }
        if let Some(card) = self.find_card_by_count(4, ignored) {
            return card;
        }
        if let Some(card) = self.find_card_by_count(3, ignored) {
            return card;
        }
        if let Some(card) = self.find_card_by_count(2, ignored) {
            return card;
        }
        CondensedCard {
            card: self.cards.iter().filter(|c| !c.is_joker()).next().cloned().unwrap_or_else(|| Card::new('J')),
            count: 1,
        }
    }

    fn find_card_by_count(&self, expected_count: u32, excluded: &[char]) -> Option<CondensedCard> {
        for card in self.cards {
            if excluded.contains(&card.label) {
                continue;
            }
            let count = (self.cards).iter().filter(|c| c.label == card.label).count() as u32;
            if count == expected_count {
                return Some(CondensedCard { card, count });
            }
        }
        None
    }

    fn find_many_cards_by_counts(&self, expected_counts: &[u32]) -> Option<Vec<CondensedCard>> {
        let mut excluded = Vec::with_capacity(5);
        let mut out = Vec::with_capacity(expected_counts.len());
        for expected_count in expected_counts {
            match self.find_card_by_count(*expected_count, &excluded) {
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

    fn match_kind(&self) -> HandMatch {
        // let counts = label_to_count.iter().map(|(&label, &count)| CondensedCard { card: Card::new(label), count }).collect_to_vec();
        // let find_by_count = |count: u32| counts.iter().filter(|c| c.count == count).next();
        // five of a kind
        if let Some(card) = self.find_card_by_count(5, &[]) {
            return HandMatch::FiveOfAKind(card.card.into());
        }
        // four of a kind
        if let Some(card) = self.find_card_by_count(4, &[]) {
            return HandMatch::FourOfAKind(card.card.into());
        }
        // full house
        if let Some(matches) = self.find_many_cards_by_counts(&[3, 2]) {
            return HandMatch::FullHouse([matches[0].card.into(), matches[1].card.into()]);
        }
        // three of a kind
        if let Some(card) = self.find_card_by_count(3, &[]) {
            return HandMatch::ThreeOfAKind(card.card.into());
        }
        // two pair
        if let Some(matches) = self.find_many_cards_by_counts(&[2, 2]) {
            return HandMatch::TwoPair([matches[0].card.into(), matches[1].card.into()]);
        }
        // one pair
        if let Some(card) = self.find_card_by_count(2, &[]) {
            return HandMatch::OnePair(card.card.into());
        }
        // high card
        if let Some(card) = self.find_many_cards_by_counts(&[1, 1, 1, 1, 1]) {
            return HandMatch::HighCard([card[0].card.into(), card[1].card.into(), card[2].card.into(), card[3].card.into(), card[4].card.into()]);
        }

        unreachable!("This should never be reached; if this is reached, please panic calmly and exit in a orderly fashion. However, if the issue is \
    that you are stuck in vim, don't bother asking for help. You are forever stuck in vim.")
    }

    fn upgraded(&self) -> Hand {
        let joker_count = self.cards.iter().filter(|card| card.is_joker()).count();
        if joker_count == 0 {
            return *self;
        }

        let most_common = self.find_most_common_card();
        Hand {
            cards: self.cards.map(|c| if c.is_joker() { most_common.card } else { c }),
            bet: self.bet,
        }
    }
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


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum HandMatch {
    FiveOfAKind(CondensedCard),
    FourOfAKind(CondensedCard),
    FullHouse([CondensedCard; 2]),
    ThreeOfAKind(CondensedCard),
    TwoPair([CondensedCard; 2]),
    OnePair(CondensedCard),
    HighCard([CondensedCard; 5]),
}

impl HandMatch {
    fn score(&self) -> u32 {
        match self {
            HandMatch::FiveOfAKind(_) => 7,
            HandMatch::FourOfAKind(_) => 6,
            HandMatch::FullHouse(_) => 5,
            HandMatch::ThreeOfAKind(_) => 4,
            HandMatch::TwoPair(_) => 3,
            HandMatch::OnePair(_) => 2,
            HandMatch::HighCard(_) => 1,
        }
    }

    #[allow(dead_code)]
    fn labels(&self) -> Vec<CondensedCard> {
        match self {
            HandMatch::FiveOfAKind(card) => vec![*card],
            HandMatch::FourOfAKind(label) => vec![*label],
            HandMatch::FullHouse(labels) => labels.to_vec(),
            HandMatch::ThreeOfAKind(label) => vec![*label],
            HandMatch::TwoPair(labels) => labels.to_vec(),
            HandMatch::OnePair(label) => vec![*label],
            HandMatch::HighCard(labels) => labels.to_vec(),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            HandMatch::FiveOfAKind(_) => "FiveOfAKind",
            HandMatch::FourOfAKind(_) => "FourOfAKind",
            HandMatch::FullHouse(_) => "FullHouse",
            HandMatch::ThreeOfAKind(_) => "ThreeOfAKind",
            HandMatch::TwoPair(_) => "TwoPair",
            HandMatch::OnePair(_) => "OnePair",
            HandMatch::HighCard(_) => "HighCard",
        }
    }
}


impl From<Card> for CondensedCard {
    fn from(card: Card) -> Self {
        CondensedCard { card, count: 1 }
    }
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

