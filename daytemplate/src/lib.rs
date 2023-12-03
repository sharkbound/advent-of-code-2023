use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

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

    fn create_day_input_directory_path(&self) -> PathBuf {
        let root_path = env!("CARGO_MANIFEST_DIR");
        let mut input_directory = PathBuf::from(root_path);
        input_directory.push("..");
        input_directory.push("inputs");
        input_directory.push(format!("{}", Self::day()));
        input_directory
    }

    fn input(&self) -> String {
        let mut input_file = self.create_day_input_directory_path();
        input_file.push("input.txt");
        let mut string = String::new();
        File::open(&input_file).expect(&format!("Could not open input file {:?}", &input_file))
            .read_to_string(&mut string).expect(&format!("Could not read input file {:?}", &input_file));
        string
    }

    fn sample(&self, id: &str) -> String {
        let mut input_file = self.create_day_input_directory_path();
        input_file.push(format!("sample_{}.txt", id));
        let mut string = String::new();
        File::open(&input_file).expect(&format!("Could not open input file {:?}", &input_file))
            .read_to_string(&mut string).expect(&format!("Could not read input file {:?}", &input_file));
        string
    }
}