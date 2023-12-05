use std::{fmt::Display, ops::Range};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, space1, u64},
    combinator::map,
    multi::{count, separated_list1},
    sequence::tuple,
    IResult,
};

use crate::days::Day;

pub struct Day05;

#[derive(Debug)]
pub enum Thing {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug)]
pub struct Almanac {
    seed_input: Vec<u64>,
    maps: Vec<Map>,
}

#[derive(Debug, PartialEq, PartialOrd, Eq)]
struct Roundtrip {
    start_seed: u64,
    end_dest: u64,
}

impl Ord for Roundtrip {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.end_dest.cmp(&other.end_dest)
    }
}

impl Almanac {
    fn map_all_seeds_p1(&self) -> Vec<Roundtrip> {
        let mut out: Vec<Roundtrip> = Vec::with_capacity(self.seed_input.len());
        for seed in &self.seed_input {
            // println!("processing seed {seed}");
            let mut cur_number = *seed;
            for map in &self.maps {
                // println!("processing map {:}", map.to_string());
                cur_number = map.map(cur_number);
            }
            out.push(Roundtrip {
                start_seed: *seed,
                end_dest: cur_number,
            })
        }
        out
    }

    fn map_all_seeds_p2(&self) -> Vec<Roundtrip> {
        let mut out: Vec<Roundtrip> = Vec::with_capacity(self.seed_input.len());
        let chunks = self.seed_input.chunks(2);
        let mut index = 1;
        for chunk in chunks {
            let seed_start = chunk[0];
            let seed_num = chunk[1];
            println!(
                "starting seed run {} of {} - {} iters",
                index,
                self.seed_input.len() / 2,
                seed_num
            );
            for seed in seed_start..(seed_num + seed_start) {
                let mut cur_number = seed;
                for map in &self.maps {
                    cur_number = map.map(cur_number);
                }
                out.push(Roundtrip {
                    start_seed: seed,
                    end_dest: cur_number,
                })
            }
            index += 1
        }
        out
    }
}

#[derive(Debug)]
pub struct Map {
    src_type: Thing,
    dest_type: Thing,
    ranges: Vec<MapRange>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:?}]->[{:?}]({} maps)",
            self.src_type,
            self.dest_type,
            self.ranges.len()
        )
    }
}

impl Map {
    fn map(&self, input: u64) -> u64 {
        for range in &self.ranges {
            let inp = range.to_input_range();
            if inp.contains(&input) {
                let offset = input - range.source_start;
                let dest = range.dest_start + offset;
                return dest;
            }
        }
        input
    }
}

#[derive(Debug)]
struct MapRange {
    dest_start: u64,
    source_start: u64,
    range_length: u64,
}

impl MapRange {
    fn to_input_range(&self) -> Range<u64> {
        self.source_start..(self.source_start + self.range_length)
    }
}

impl Day05 {
    fn parse_init_seeds(input: &str) -> IResult<&str, Vec<u64>> {
        map(
            tuple((tag("seeds: "), separated_list1(space1, u64))),
            |(_, x)| x,
        )(input)
    }

    fn parse_map_desc(input: &str) -> IResult<&str, (Thing, Thing)> {
        map(
            tuple((
                Self::parse_thing_word,
                tag("-to-"),
                Self::parse_thing_word,
                tag(" map:"),
            )),
            |x| (x.0, x.2),
        )(input)
    }

    fn parse_thing_word(input: &str) -> IResult<&str, Thing> {
        map(
            alt((
                tag("seed"),
                tag("soil"),
                tag("fertilizer"),
                tag("water"),
                tag("light"),
                tag("temperature"),
                tag("humidity"),
                tag("location"),
            )),
            |t| match t {
                "seed" => Thing::Seed,
                "soil" => Thing::Soil,
                "fertilizer" => Thing::Fertilizer,
                "water" => Thing::Water,
                "light" => Thing::Light,
                "temperature" => Thing::Temperature,
                "humidity" => Thing::Humidity,
                "location" => Thing::Location,
                _ => unreachable!(),
            },
        )(input)
    }

    fn parse_single_range(input: &str) -> IResult<&str, MapRange> {
        map(tuple((u64, space1, u64, space1, u64)), |t| MapRange {
            dest_start: t.0,
            source_start: t.2,
            range_length: t.4,
        })(input)
    }

    fn parse_map_block(input: &str) -> IResult<&str, Map> {
        map(
            tuple((
                Self::parse_map_desc,
                line_ending,
                separated_list1(line_ending, Self::parse_single_range),
            )),
            |(m, _, rangelist)| Map {
                src_type: m.0,
                dest_type: m.1,
                ranges: rangelist,
            },
        )(input)
    }
}

impl Day for Day05 {
    type Input = Almanac;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        map(
            tuple((
                Self::parse_init_seeds,
                count(line_ending, 2),
                separated_list1(count(line_ending, 2), Self::parse_map_block),
            )),
            |t| Almanac {
                seed_input: t.0,
                maps: t.2,
            },
        )(input)
    }

    type Output1 = u64;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .map_all_seeds_p1()
            .iter()
            .map(|x| x.end_dest)
            .sorted()
            .next()
            .unwrap()
    }

    type Output2 = u64;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        input
            .map_all_seeds_p2()
            .iter()
            .map(|x| x.end_dest)
            .sorted()
            .next()
            .unwrap()
    }
}
