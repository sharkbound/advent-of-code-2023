use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub enum DayPart {
    ONE,
    TWO,
}

impl DayPart {
    fn as_str(&self) -> &'static str {
        match self {
            DayPart::ONE => "1",
            DayPart::TWO => "2",
        }
    }

    fn as_int(&self) -> i32 {
        match self {
            DayPart::ONE => 1,
            DayPart::TWO => 2,
        }
    }
}

pub trait Day {
    type ParseOutput;

    fn part() -> DayPart;
    fn day() -> i32;
    fn parse(&self, input: &str) -> Self::ParseOutput;
    fn solve(&self);

    fn input(&self) -> String {
        let day = Self::day();
        let root_path = env!("CARGO_MANIFEST_DIR");
        let mut input_file = PathBuf::from(root_path);
        input_file.push("..");
        input_file.push("inputs");
        input_file.push(format!("{}", day));
        input_file.push("input.txt");
        let mut string = String::new();
        File::open(&input_file).expect(&format!("Could not open input file {:?}", &input_file))
            .read_to_string(&mut string).expect(&format!("Could not read input file {:?}", &input_file));
        string
    }
}