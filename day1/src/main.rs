use daytemplate::Day;
use crate::day_1_part_1::Day1Part1;
use crate::day_1_part_2::Day1Part2;

mod day_1_part_1;
mod day_1_part_2;

fn main() {
    Day1Part1::new().solve();
    Day1Part2::new().solve();
}
