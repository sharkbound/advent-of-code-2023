use daytemplate::{Day, DayPart};

pub struct Day9Part1;

impl Day for Day9Part1 {
    type ParseOutput = Vec<Vec<u32>>;

    fn part() -> DayPart {
        DayPart::ONE
    }

    fn day() -> i32 {
        9
    }

    fn parse(&self, input: &str) -> Self::ParseOutput {
        input.lines()
            .map(|line|
                line.split(' ')
                    .flat_map(|x| x.parse::<u32>())
                    .collect::<Vec<u32>>()
            ).collect()
    }

    fn solve(&self) {
        // let input = self.input();
        let input = self.sample("part_1");
        let parsed = self.parse(&input);

        for line in parsed.iter() {
            let diffs = generate_differences(line);
            println!("DIFFS: {:?}", diffs);
            for i in (1..diffs.len()).rev() {
                let first = &diffs[i];
                let second = &diffs[i - 1];
                let step_first = find_step(first);
                let step_second = find_step(second);
                println!("{:?} -> {} | {:?} -> {}", first, step_first, second, step_second);
            }
        }
    }
}

fn diff_once(data: &Vec<u32>) -> Vec<u32> {
    let mut diffs = Vec::with_capacity(data.len() - 1);
    for i in 0..data.len() - 1 {
        diffs.push(data[i].abs_diff(data[i + 1]));
    }
    diffs
}

fn generate_differences(data: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut out = Vec::new();
    let mut current_diffs = data.clone();
    out.push(current_diffs.clone());
    loop {
        let diffs = diff_once(&current_diffs);
        out.push(diffs.clone());
        if diffs_are_zero(&diffs) {
            return out;
        }
        current_diffs = diffs;
    }
}

fn diffs_are_zero(diffs: &[u32]) -> bool {
    diffs.iter().all(|&x| x == 0)
}

#[derive(Debug)]
struct StepValues {
    changes: Vec<u32>,
    diff_between_changes: u32,
}

impl StepValues {
    fn new(values: Vec<u32>) -> Self {
        let mut changes = Vec::new();
        for i in 1..values.len() {
            let prev = values[i - 1];
            let curr = values[i];
            changes.push(curr.abs_diff(prev));
        }
        Self {
            diff_between_changes: match &changes[..] {
                [val ] => *val,
                [v1, v2, ..] => v1.abs_diff(*v2),
                [] => 0,
            },
            changes,
        }
    }
}

fn find_step(data: &[u32]) -> u32 {
    match data {
        [first, second] => first.abs_diff(*second),
        [first, second, third, ..] => {
            let diff_first_second = first.abs_diff(*second);
            let diff_second_third = second.abs_diff(*third);
            if diff_first_second == diff_second_third {
                return diff_first_second;
            }
            first.abs_diff(*second).abs_diff(second.abs_diff(*third))
        }
        _ => 0
    }
}

fn first_and_second(data: &[u32]) -> (u32, u32) {
    let first = data.first().unwrap();
    let second = data.get(1).unwrap_or(data.first().unwrap());
    (*first, *second)
}


/*
[[10, 13, 16, 21, 30, 45], 
[3, 3, 5, 9, 15], 
[0, 2, 4, 6], 
[2, 2, 2], 
[0, 0]]

10  13  16  21  30  45  68
   3   3   5   9  15  23
     0   2   4   6   8
       2   2   2   2
         0   0   0
*/