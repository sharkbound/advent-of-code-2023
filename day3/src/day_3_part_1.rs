use std::ops::Range;
use ndarray::Array2;
use daytemplate::DayPart;
use rustutils::collections::CollectToVec;
use rustutils::iterable_string_ext::JoinToStringExt;

pub struct Day3Part1 {}

impl Day3Part1 {
    pub fn new() -> Self {
        Day3Part1 {}
    }
}

impl daytemplate::Day for Day3Part1 {
    type ParseOutput = Array2<GridValue>;

    fn part() -> DayPart {
        DayPart::ONE
    }

    fn day() -> i32 {
        3
    }

    fn parse(&self, input: &str) -> Self::ParseOutput {
        let lines = input.lines().collect_to_vec();
        let mut arr = Array2::from_elem((lines.len(), lines[0].len()), GridValue::EMPTY);
        for (i, line) in lines.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                arr[[i, j]] = match c {
                    '.' => GridValue::EMPTY,
                    number if number.is_digit(10) => GridValue::NUMBER(number.to_digit(10).unwrap(), number),
                    symbol => GridValue::SYMBOL(symbol),
                };
            }
        }
        arr
    }

    fn solve(&self) {
        let input = self.sample("part_1");
        let parsed = self.parse(&input);
        let processed = process_array(&parsed);

        let mut total = 0;
        let mut remaining_numbers = processed.numbers.clone();
        for (_, index) in processed.symbols {
            let matches = remaining_numbers.iter().enumerate().filter(|(_, number_match)| {
                (number_match.start[1]..number_match.end[1])
                    .any(|col|
                        are_indexes_adjacent([number_match.start[0], col], index)
                    )
            }).map(|x| x.clone()).collect_to_vec();

            total += matches.iter().fold(0, |acc, (_, number_match)| acc + number_match.number);
            for (index, _) in matches.iter().rev() {
                remaining_numbers.remove(*index); // fixme
            }
        }
        println!("Day 3 Part 1: {}", total);
    }
}

fn are_indexes_adjacent(a: [usize; 2], b: [usize; 2]) -> bool {
    (a[0] as i32 + a[1] as i32).abs() <= 1 && (b[0] as i32 + b[1] as i32) <= 1
}

fn process_array(arr: &Array2<GridValue>) -> ParsedArray {
    let mut symbols = Vec::new();
    let mut numbers = Vec::new();
    let (mut row, mut col) = (0, 0);
    while row < arr.nrows() {
        while col < arr.ncols() {
            let index = [row, col];
            let value = arr[index];
            match value {
                GridValue::SYMBOL(symbol) => symbols.push((symbol, index)),
                GridValue::NUMBER(_, _) => {
                    let ret = read_whole_number(&arr, index);
                    col = ret.end[1];
                    numbers.push(ret);
                    continue;
                }
                _ => {}
            }
            col += 1;
        }
        row += 1;
        col = 0;
    }

    ParsedArray { symbols, numbers }
}

struct ParsedArray {
    symbols: Vec<(char, [usize; 2])>,
    numbers: Vec<WholeNumberSearchResult>,
}

#[derive(Debug, Clone)]
struct WholeNumberSearchResult {
    number: u32,
    start: [usize; 2],
    end: [usize; 2],
}

fn read_whole_number(arr: &Array2<GridValue>, start: [usize; 2]) -> WholeNumberSearchResult {
    let mut numbers = Vec::new();
    let mut i = start[1];
    while i < arr.ncols() {
        let value = arr[[start[0], i]];
        if let GridValue::NUMBER(_, char) = value {
            numbers.push(char);
        } else {
            break;
        }
        i += 1;
    }

    WholeNumberSearchResult {
        number: numbers.iter().join_to_string("", |x| x.to_string()).parse().unwrap(),
        start,
        end: [start[0], i],
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GridValue {
    NUMBER(u32, char),
    EMPTY,
    SYMBOL(char),
}

