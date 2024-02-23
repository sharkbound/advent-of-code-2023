use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace0};
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{delimited, tuple};
use rustc_hash::FxHashMap;
use daytemplate::{Day, DayPart};
use rustutils::nom_helpers::consume_empty_space;

pub(crate) struct Day8Part1;

impl<'a> Day for Day8Part1 {
    type ParseOutput = (Vec<Move>, Vec<ParsedNode>);

    fn part() -> DayPart {
        DayPart::ONE
    }

    fn day() -> i32 {
        8
    }

    fn parse(&self, input: &str) -> Self::ParseOutput {
        let (_, data) = nom_parse(input).unwrap();
        data
    }

    fn solve(&self) {
        // let input = self.input();
        let input = self.sample("part_1");
        let parsed = self.parse(&input);

        let mut moves = parsed.0;
        let nodes = parsed.1;

        let mut node_connections = FxHashMap::default();
        for node in nodes.iter() {
            node_connections.insert(node.node_id(), NodeConnections { left: node.node_left(), right: node.node_right() });
        }
        
        let mut current_node = node_connections.iter().filter(|(node, _)| node.id == "AAA").next().unwrap().0;
        let mut jumps = 0u32;
        let mut idx = 0;
        while current_node != &"ZZZ" {
            jumps += 1;
            let connections = node_connections.get(&current_node).unwrap();
            let current_move = moves[idx];
            
            let next_node = match current_move {
                Move::Left => &connections.left,
                Move::Right => &connections.right,
            };
            
            if current_node != next_node {
                println!("Current Node: {:?}, Current Move: {:?} Next Node: {:?}", current_node.id, current_move, next_node.id);
            }
            
            current_node = next_node;
            idx = (idx + 1) % moves.len();
        }
        
        println!("Day 8 Part 1: {}", jumps);
    }
}

#[derive(Debug)]
struct NodeConnections<'a> {
    left: Node<'a>,
    right: Node<'a>,
}

#[derive(Debug, Copy, Clone)]
pub enum Move {
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

fn nom_parse(input: &str) -> IResult<&str, (Vec<Move>, Vec<ParsedNode>)> {
    let (input, _moves) = nom_parse_movements(input)?;
    let (input, _) = consume_empty_space(input)?;
    let (input, _connections) = nom_parse_connections(input)?;
    Ok((input, (_moves, _connections)))
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Node<'a> {
    id: &'a str,
}

impl PartialEq<&str> for Node<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.id == *other
    }
}

#[derive(Debug)]
pub struct ParsedNode {
    id: String,
    left: String,
    right: String,
}

impl ParsedNode {
    fn new(id: &str, left: &str, right: &str) -> Self {
        ParsedNode {
            id: id.to_owned(),
            left: left.to_owned(),
            right: right.to_owned(),
        }
    }

    fn node_left(&self) -> Node {
        Node { id: &self.left }
    }

    fn node_right(&self) -> Node {
        Node { id: &self.id }
    }

    fn node_id(&self) -> Node {
        Node { id: &self.id }
    }
}

fn nom_parse_connections(input: &str) -> IResult<&str, Vec<ParsedNode>> {
    Ok(many1(
        map_res(
            tuple((
                alpha1, multispace0, tag("="), multispace0, delimited(tag("("), nom_parse_left_right_connections, tag(")")), consume_empty_space,
            )),
            |(id, _, _, _, (left, right), _)| Result::<ParsedNode, ()>::Ok(ParsedNode::new(id, left, right)),
        )
    )(input)?)
}

fn nom_parse_left_right_connections(input: &str) -> IResult<&str, (&str, &str)> {
    Ok(
        map_res(
            tuple((
                alpha1, tag(","), multispace0, alpha1
            )),
            |x| Result::<(&str, &str), ()>::Ok((x.0, x.3)),
        )(input)?
    )
}