use clap::builder::OsStr;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, newline, not_line_ending, space1, u64},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{separated_pair, tuple},
    IResult,
};

use crate::days::Day;

pub struct Day02;

#[derive(Debug, Default)]
pub struct Observation {
    red: u64,
    green: u64,
    blue: u64,
}

impl Observation {
    fn is_valid_part1(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn power(&self) -> u64 {
        self.blue * self.green * self.red
    }
}

impl Day02 {
    fn game_parts(input: &str) -> IResult<&str, Observation> {
        map(
            separated_list1(
                tag(", "),
                separated_pair(u64, space1, alt((tag("blue"), tag("green"), tag("red")))),
            ),
            |part| {
                let mut obs: Observation = Observation::default();
                for (num, color) in part {
                    match color {
                        "green" => obs.green = num,
                        "red" => obs.red = num,
                        "blue" => obs.blue = num,
                        _ => unreachable!("not a color?"),
                    }
                }
                obs
            },
        )(input)
    }
    fn single_game(input: &str) -> IResult<&str, Game> {
        map(
            tuple((
                tag("Game "),
                u64,
                tag(":"),
                space1,
                separated_list1(tag("; "), Self::game_parts),
            )),
            |f| Game {
                index: f.1,
                observations: f.4,
            },
        )(input)
    }
}

#[derive(Debug)]
pub struct Game {
    index: u64,
    observations: Vec<Observation>,
}

impl Game {
    fn possible_part1(&self) -> bool {
        self.observations.iter().all(|obs| obs.is_valid_part1())
    }

    fn minimum_cubes(&self) -> Observation {
        let mut running_min = Observation::default();
        for obs in self.observations.iter() {
            if obs.blue > running_min.blue {
                running_min.blue = obs.blue
            }
            if obs.green > running_min.green {
                running_min.green = obs.green
            }
            if obs.red > running_min.red {
                running_min.red = obs.red
            }
        }
        running_min
    }
}

impl Day for Day02 {
    type Input = Vec<Game>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list1(newline, Self::single_game)(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .filter(|p| p.possible_part1())
            .map(|t| t.index as usize)
            .sum()
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        input
            .iter()
            .map(|s| s.minimum_cubes())
            .map(|t| t.power() as usize)
            .sum()
    }
}
