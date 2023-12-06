use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::{digit1, multispace0};
use nom::IResult;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use daytemplate::{Day, DayPart};
use rustutils::collections::CollectToVec;

pub struct Day2Part1;

impl Day2Part1 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Day for Day2Part1 {
    type ParseOutput = Vec<Game>;

    fn part() -> DayPart {
        DayPart::ONE
    }

    fn day() -> i32 {
        2
    }

    fn parse(&self, input: &str) -> Self::ParseOutput {
        input.lines().flat_map(nom_parse_line).map(|(_, game)| game).collect_to_vec()
    }

    fn solve(&self) {
        // let input = self.sample("part_1");
        let input = self.input();
        let parsed = self.parse(&input);
        let player_bag = player_bag();
        let total = parsed
            .iter()
            .filter(|game|
                game.bags
                    .iter()
                    .all(|bag|
                        check_playability(&player_bag, bag))
            )
            .map(|game| game.id)
            .sum::<u32>();
        println!("Day 2 Part 1: {}", total);
    }
}

#[derive(Debug, Default)]
struct CubeBag {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

fn check_playability(player_bag: &CubeBag, game: &CubeBag) -> bool {
    player_bag.red >= game.red && player_bag.green >= game.green && player_bag.blue >= game.blue
}

fn player_bag() -> CubeBag {
    // 12 red cubes, 13 green cubes, and 14 blue cubes
    CubeBag {
        red: 12,
        green: 13,
        blue: 14,
    }
}


fn nom_game_id(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag_no_case("game")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, id) = digit1(input)?;
    Ok((input, id.parse().unwrap()))
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

fn nom_color_value(input: &str) -> IResult<&str, (Color, u32)> {
    let (input, _) = multispace0(input)?;
    let (input, (count, _, color)): (&str, (&str, &str, &str)) = tuple((
        digit1,
        multispace0,
        alt((tag_no_case("red"), tag_no_case("blue"), tag_no_case("green")))
    ))(input)?;

    let color = match color {
        "red" => Color::Red,
        "blue" => Color::Blue,
        "green" => Color::Green,
        _ => unreachable!(),
    };

    Ok((input, (color, count.parse().unwrap())))
}

fn nom_bag(input: &str) -> IResult<&str, CubeBag> {
    let (input, matches) = separated_list0(tag(","), nom_color_value)(input)?;
    let mut bag = CubeBag::default();
    for (color, count) in matches {
        match color {
            Color::Red => bag.red += count,
            Color::Green => bag.green += count,
            Color::Blue => bag.blue += count,
        }
    }
    Ok((input, bag))
}

#[derive(Debug)]
pub struct Game {
    id: u32,
    bags: Vec<CubeBag>,
}

fn nom_parse_line(input: &str) -> IResult<&str, Game> {
    let (input, id) = nom_game_id(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, bags) = separated_list0(tag(";"), nom_bag)(input)?;
    Ok((input, Game { id, bags }))
}