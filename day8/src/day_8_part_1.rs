use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace0, multispace1};
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{delimited, tuple};
use daytemplate::{Day, DayPart};
use rustutils::nom_helpers::consume_empty_space;

pub(crate) struct Day8Part1;

impl Day for Day8Part1 {
    type ParseOutput = ();

    fn part() -> DayPart {
        DayPart::ONE
    }

    fn day() -> i32 {
        8
    }

    fn parse(&self, input: &str) -> Self::ParseOutput {
        let (_, data) = nom_parse(input).unwrap();
    }

    fn solve(&self) {
        // let input = self.input();
        let input = self.sample("part_1");
        let parsed = self.parse(&input);
    }
}

#[derive(Debug, Copy, Clone)]
enum Move {
    Left,
    Right,
}

fn nom_parse_movements(input: &str) -> IResult<&str, Vec<Move>> {
    Ok(
        many1(
            map_res(
                alt((tag("L"), tag("R"))),
                |m: &str| Result::<Move, ()>::Ok(match m {
                    "L" => Move::Left,
                    "R" => Move::Right,
                    _ => unreachable!(),
                }),
            )
        )(input)?
    )
}

fn nom_parse(input: &str) -> IResult<&str, ()> {
    let (input, result) = nom_parse_movements(input)?;
    let (input, _) = consume_empty_space(input)?;
    let (input, conns) = nom_parse_connections(input)?;
    println!("{:?}", conns);
    Ok((input, ()))
}

#[derive(Debug)]
struct ParsedNode<'a> {
    id: &'a str,
    left: &'a str,
    right: &'a str,
}

fn nom_parse_connections(input: &str) -> IResult<&str, Vec<ParsedNode>> {
    Ok(many1(
        map_res(
            tuple((
                alpha1, multispace0, tag("="), multispace0, delimited(tag("("), nom_parse_left_right_connections, tag(")")), consume_empty_space,
            )),
            |(id, _, _, _, (left, right), _)| Result::<ParsedNode, ()>::Ok(ParsedNode { id, left, right }),

        )
    )(input)?)
}

fn nom_parse_left_right_connections(input: &str) -> IResult<&str, (&str, &str)> {
    Ok(
        map_res(
            tuple((
                alpha1, tag(","), multispace0, alpha1
            )),
            |x| Result::<(&str, &str), ()>::Ok((x.0, x.3))
        )(input)?
    )
}