use std::collections::{BinaryHeap, HashSet};
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
        // let input = self.sample("part_1");
        let input = self.input();

        let parsed = self.parse(&input);
        let processed = process_array(&parsed);

        let mut numbers_array = Array2::from_elem(array_shape::<2, _>(&parsed), 0u32);
        for WholeNumberSearchResult { start, end, number } in processed.numbers.iter() {
            let row = start[0];
            for col in start[1]..end[1] {
                numbers_array[[row, col]] = *number;
            }
        }

        let mut total = 0u32;
        for (_, index) in processed.symbols.iter() {
            total += find_numbers_around_index(&numbers_array, index).iter().sum::<u32>();
        }
        println!("Day 3 Part 1: {}", total);
    }
}

fn is_index_valid(shape: &[usize; 2], index: [usize; 2]) -> bool {
    let [row, col] = index;
    let [height, width] = shape;
    return row < *height && col < *width;
}

fn find_numbers_around_index(arr: &Array2<u32>, search_origin: &[usize; 2]) -> HashSet<u32> {
    let shape = array_shape::<2, _>(arr);
    let [sy, sx] = search_origin;
    let mut numbers = HashSet::new();
    for yoffset in -1_i32..=1 {
        for xoffset in -1_i32..=1 {
            if let (Ok(y), Ok(x)) = (usize::try_from(*sy as i32 + yoffset), usize::try_from(*sx as i32 + xoffset)) {
                let index = [y, x];
                if !is_index_valid(&shape, index) {
                    continue;
                }
                let value = arr[index];
                if value != 0 {
                    numbers.insert(value);
                }
            }
        }
    }
    numbers
}

fn array_shape<const dims: usize, T>(arr: &Array2<T>) -> [usize; dims] {
    let mut out: [usize; dims] = [0; dims];
    let shape = arr.shape();
    for i in 0..dims {
        out[i] = shape[i];
    }
    out
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

#[derive(Debug, Clone, Copy)]
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

