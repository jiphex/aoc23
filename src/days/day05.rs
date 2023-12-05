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
    fn map_all_seeds_p1(&self) -> u64 {
        let mut lowest_seen = (u64::MAX, u64::MAX);
        for seed in &self.seed_input {
            let mut cur_number = *seed;
            for map in &self.maps {
                cur_number = map.map(cur_number);
            }
            if cur_number < lowest_seen.0 {
                lowest_seen = (cur_number, *seed);
            }
        }
        lowest_seen.1
    }

    fn map_all_seeds_p2(&self) -> u64 {
        let mut lowest_seen = (u64::MAX, u64::MAX);
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
                if cur_number < lowest_seen.0 {
                    lowest_seen = (cur_number, seed)
                }
            }
            index += 1
        }
        lowest_seen.1
    }

    /// Alternative implementation for p2 that should be faster (maybe..):
    ///
    /// 1. First, reverse the list of maps so we can do a backwards scan through
    ///    them, starting with the location map.
    /// 2. Then sort the location map so that we know which is the lowest
    ///    destination numbers
    /// 3. Trace through the first location map range (starting at 0) and map
    ///    this all the way back through the full map (backwards) to get an
    ///    input range of seeds that would map to that location
    /// 4. Check the input list of seeds to see if any match
    /// 5. Check the interim list of map ranges between this and the next to
    ///    check if any seeds fit into that gap
    /// 6. Move otni the next range, continue from (3) above until we find a
    ///    seed that either fits one of the ranges, or one of the gaps
    ///    in-between, then return it
    fn map_all_seeds_p2_reverse(&self) -> u64 {
        let rev_maps: Vec<&Map> = self.maps.iter().rev().collect();
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

#[derive(Debug, PartialEq, PartialOrd, Eq)]
struct MapRange {
    dest_start: u64,
    source_start: u64,
    range_length: u64,
}

mod test {
    use super::MapRange;

    #[test]
    fn test_ordering() {
        let res_a = MapRange {
            dest_start: 10,
            source_start: 1000,
            range_length: 0,
        };
        let res_b = MapRange {
            dest_start: 20,
            source_start: 0,
            range_length: 0,
        };
        assert!(res_b > res_a);
        assert!(res_a < res_b);
        let res_c: Option<MapRange> = None;
        let res_d = MapRange {
            dest_start: 10,
            source_start: 0,
            range_length: 0,
        };
        assert!(res_c < Some(res_d));
    }
}

impl Ord for MapRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dest_start.cmp(&other.dest_start)
    }
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
        input.map_all_seeds_p1()
    }

    type Output2 = u64;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        input.map_all_seeds_p2()
    }
}
