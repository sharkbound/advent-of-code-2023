use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, multispace0};
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{delimited, tuple};
use rustc_hash::FxHashMap;
use daytemplate::{Day, DayPart};
use rustutils::nom_helpers::consume_empty_space;


pub(crate) struct Day8Part2;

impl<'a> Day for Day8Part2 {
    type ParseOutput = (Vec<Move>, Vec<ParsedNode>);

    fn part() -> DayPart {
        DayPart::TWO
    }

    fn day() -> i32 {
        8
    }

    fn parse(&self, input: &str) -> Self::ParseOutput {
        let (_, data) = nom_parse(input).unwrap();
        data
    }
    /*
    If you were a ghost, 
    you'd probably just start at every node that ends with A and follow all the paths
     at the same time until they all simultaneously end up at nodes that end with Z.
     */
    fn solve(&self) {
        let input = self.input();
        // let input = self.sample("part_2_1");
        let parsed = self.parse(&input);

        let moves = parsed.0;
        let nodes = parsed.1;

        let mut node_connections = FxHashMap::default();
        for node in nodes.iter() {
            node_connections.insert(node.node_id(), NodeConnections { left: node.node_left(), right: node.node_right() });
        }

        let mut current_nodes = node_connections
            .iter()
            .filter(|(node, _)| node.id.ends_with("A"))
            .map(|x| x.0)
            .collect::<Vec<_>>();

        let mut move_idx = 0;
        let mut jumps = 0;
        let mut dist_to_z = Vec::new();
        loop {
            let changes = advance(moves[move_idx], &mut current_nodes, &node_connections);
            // println!("{}({})[{}] : {:?}", jumps, changes.change_count, changes.new_z_nodes, current_nodes);
            if changes.change_count == 0 {
                break;
            }
            jumps += 1;
            if changes.new_z_nodes != 0 {
                dist_to_z.push(jumps);
            }
            move_idx = (move_idx + 1) % moves.len();
        }
        println!("All Z Distances: {:?}", dist_to_z);
        println!("Day 8 Part 2: {}", dist_to_z.iter().fold(1, |acc, &x| lcm(acc, x)));
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == b { return a; }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

fn lcm(a: u64, b: u64) -> u64 {
    return a * (b / gcd(a, b));
}

fn advance<'a>(move_: Move, current_nodes: &mut [&NodeName<'a>], connections: &'a FxHashMap<NodeName<'a>, NodeConnections>) -> Change {
    let mut change_count = 0;
    let mut new_z_nodes = 0;
    for node in current_nodes.iter_mut() {
        if node.id.ends_with("Z") {
            continue;
        }
        let connections = connections.get(node).unwrap();
        let new_node = match move_ {
            Move::Left => &connections.left,
            Move::Right => &connections.right,
        };
        *node = new_node;
        change_count += 1;
        if new_node.id.ends_with("Z") {
            new_z_nodes += 1;
        }
    }
    Change { change_count, new_z_nodes }
}

struct Change {
    new_z_nodes: u32,
    change_count: u32,
}

#[derive(Debug)]
struct NodeConnections<'a> {
    left: NodeName<'a>,
    right: NodeName<'a>,
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
struct NodeName<'a> {
    id: &'a str,
}

impl NodeName<'_> {
    fn id_equals(&self, other: &str) -> bool {
        self.id == other
    }
}

// impl PartialEq<&str> for Node<'_> {
//     fn eq(&self, other: &&str) -> bool {
//         self.id == *other
//     }
// }

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

    fn node_left(&self) -> NodeName {
        NodeName { id: &self.left }
    }

    fn node_right(&self) -> NodeName {
        NodeName { id: &self.right }
    }

    fn node_id(&self) -> NodeName {
        NodeName { id: &self.id }
    }
}

fn nom_parse_connections(input: &str) -> IResult<&str, Vec<ParsedNode>> {
    Ok(many1(
        map_res(
            tuple((
                alphanumeric1, multispace0, tag("="), multispace0, delimited(tag("("), nom_parse_left_right_connections, tag(")")), consume_empty_space,
            )),
            |(id, _, _, _, (left, right), _)| Result::<ParsedNode, ()>::Ok(ParsedNode::new(id, left, right)),
        )
    )(input)?)
}


fn nom_parse_left_right_connections(input: &str) -> IResult<&str, (&str, &str)> {
    Ok(
        map_res(
            tuple((
                alphanumeric1, tag(","), multispace0, alphanumeric1
            )),
            |x| Result::<(&str, &str), ()>::Ok((x.0, x.3)),
        )(input)?
    )
}

