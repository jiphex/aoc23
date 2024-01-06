use std::{collections::BTreeMap, path::Display};

use itertools::Itertools;
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

struct Follower<'lt> {
    start_at: String,
    current_node: Option<String>,
    map_ref: &'lt FlatMap<'lt>,
    step_count: usize,
}

impl<'a, 'lt> Follower<'lt> {
    pub fn new(start_at: &'a str, map: &'lt FlatMap) -> Self {
        Self {
            start_at: start_at.to_string(),
            map_ref: map,
            current_node: None,
            step_count: 0,
        }
    }

    pub fn step(&mut self, using: &Direction) -> bool {
        if self.current_node.is_none() {
            self.current_node = Some(self.start_at.clone());
        }
        let targets = self
            .map_ref
            .get(&self.current_node.as_ref().unwrap().as_str())
            .expect("unable to find node in map");
        let next_step = match using {
            Direction::Left => targets.0,
            Direction::Right => targets.1,
        };
        self.current_node = Some(next_step.to_string());
        self.step_count += 1;
        self.is_completed()
    }

    pub fn is_completed(&self) -> bool {
        self.current_node.as_ref().is_some_and(|c| c.ends_with('Z'))
    }

    pub fn step_count(&self) -> usize {
        self.step_count
    }
}

impl std::fmt::Debug for Follower<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Follower")
            .field("start_at", &self.start_at)
            .field("current_node", &self.current_node)
            .field("map_ref", &self.map_ref)
            .field("step_count", &self.step_count)
            .finish()
    }
}

// fn count_partitions<T>(input: (Vec<T>,Vec<T>)) -> (usize,usize) {
// }

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
        let mut running_items: Vec<Follower> = input
            .nodes
            .iter()
            .map(|c| c.node.as_str())
            .filter(|s| s.ends_with('A'))
            .map(|c| {Follower::new(c, &map)})
            .collect();
        for dir in input.lr_instructions.iter().cycle() {
            // let (a,b) = running_items.iter_mut().part(|c|c.step(dir)); {
                // println!("all done")
            // }
        }
        0_usize
    }
}
