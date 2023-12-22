use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, line_ending},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

use crate::days::Day;

pub struct Day08;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

fn sided<'a, T>(source: &'a (&T, &T), pick: &Direction) -> &'a T
where
    T: ?Sized,
{
    match pick {
        Direction::Left => source.0,
        Direction::Right => source.1,
    }
}

#[derive(Debug)]
pub struct Node {
    node: String,
    direction: (String, String),
}

#[derive(Debug)]
pub struct Map {
    lr_instructions: Vec<Direction>,
    nodes: Vec<Node>,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    map(alt((char('L'), char('R'))), |c| match c {
        c if c == 'L' => Direction::Left,
        c if c == 'R' => Direction::Right,
        _ => unreachable!(),
    })(input)
}

fn parse_lrinst(input: &str) -> IResult<&str, Vec<Direction>> {
    many1(parse_direction)(input)
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    map(
        tuple((
            alpha1,
            tag(" = "),
            delimited(
                char('('),
                separated_pair(alpha1, tag(", "), alpha1),
                char(')'),
            ),
        )),
        |(snode, _, sdirection)| {
            let s: &str = snode;
            let dir: (&str, &str) = sdirection;
            Node {
                node: s.to_string(),
                direction: (dir.0.to_string(), dir.1.to_string()),
            }
        },
    )(input)
}

fn parse_nodes(input: &str) -> IResult<&str, Vec<Node>> {
    separated_list1(line_ending, parse_node)(input)
}

type FlatMap<'a> = BTreeMap<&'a str, (&'a str, &'a str)>;

fn create_btm(input: &Map) -> FlatMap {
    let mut map: FlatMap = BTreeMap::new();
    for node in input.nodes.iter() {
        map.insert(&node.node, (&node.direction.0, &node.direction.1));
    }
    map
}

impl Day for Day08 {
    type Input = Map;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        map(
            tuple((parse_lrinst, line_ending, line_ending, parse_nodes)),
            |(lr_instructions, _, _, nodes)| Map {
                lr_instructions,
                nodes,
            },
        )(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        // return 0;
        let map = create_btm(input);
        let mut next_node = "AAA";
        let mut cycle_count = 0;
        for dir in input.lr_instructions.iter().cycle() {
            next_node = sided(map.get(next_node).unwrap(), dir);
            println!("S {next_node}");
            cycle_count += 1;
            if next_node.ends_with('Z') {
                break;
            }
        }
        cycle_count
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let map = create_btm(input);
        let mut running_items: Vec<(&str, &str, usize)> = input
            .nodes
            .iter()
            .filter(|c| c.node.ends_with("A"))
            .map(|t| (t.node.as_str(), t.node.as_str(), 0))
            .collect();
        let mut cycle_count = 0;
        let mut completed_seen: usize = 0;
        for dir in input.lr_instructions.iter().cycle() {
            for (start_node, next_node, iters) in running_items.iter_mut() {
                let new_next_node = sided(map.get(next_node).unwrap(), dir);
                *next_node = new_next_node;
                if next_node.ends_with("Z") {
                    // println!(
                    //     // "node {start_node} moved to end state {next_node} in {iters} iterations"
                    // );
                } else {
                    // println!("node {start_node} is still moving towards its end state, currently at {next_node} in {iters} iterations");
                    *iters += 1;
                }
            }
            // println!("map {running_items:?}");
            cycle_count += 1;
            let completed = running_items.iter().filter(|i| i.1.ends_with("Z")).count();
            if completed == running_items.len() {
                println!("all items completed");
                break;
            } else {
                if completed > completed_seen {
                    println!("{:?}/{} items at end state", completed, running_items.len());
                    completed_seen = completed;
                }
            }
        }
        cycle_count
    }
}
