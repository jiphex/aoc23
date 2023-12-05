

use nom::{
    character::complete::{line_ending, not_line_ending},
    combinator::{map},
    multi::{separated_list1},
    IResult,
};

use crate::days::Day;

pub struct Day03;

#[derive(Clone, Copy)]
struct Coordinate {
    row: usize,
    column: usize,
}

impl Coordinate {
    fn new(row: Option<usize>, column: Option<usize>) -> Option<Coordinate> {
        if let (Some(row), Some(column)) = (row, column) {
            Some(Coordinate { row, column })
        } else {
            None
        }
    }
}

type Field = Vec<Vec<char>>;

struct FieldSize {
    rows: usize,
    columns: usize,
}

fn char_at(field: &Field, target: &Option<Coordinate>) -> Option<char> {
    if let Some(target) = target {
        let row = field.get(target.row)?;
        row.get(target.column).copied()
    } else {
        None
    }
}

fn field_dimensions(field: &Field) -> FieldSize {
    FieldSize {
        rows: field.first().unwrap_or(&Vec::<char>::new()).len(),
        columns: field.len(),
    }
}

fn capped_increment(lhs: usize, cap: usize) -> Option<usize> {
    if lhs + 1 == cap {
        None
    } else {
        Some(lhs + 1)
    }
}

// fn row_ranges(row: &Vec<char>) -> Vec<Range<usize>> {
//     let row: String = row.iter().collect();
//     for (i, char) in row.chars().enumerate() {
//         if char.is_numeric() {
//             let number: u32 = u32::<&str, nom::error::Error<_>>(&row[i..]).unwrap().1;
//         }
//     }
//     todo!()
// }

fn collides_with(field: &Field, target: &Coordinate) -> [Option<Coordinate>; 8] {
    let field_size = field_dimensions(field);
    let left = target.column.checked_sub(1);
    let col = Some(target.column);
    let right = capped_increment(target.column, field_size.columns);
    let up = target.row.checked_sub(1);
    let row = Some(target.row);
    let down = capped_increment(target.row, field_size.rows);
    [
        Coordinate::new(up, left),
        Coordinate::new(up, col),
        Coordinate::new(up, right),
        Coordinate::new(row, left),
        Coordinate::new(row, right),
        Coordinate::new(down, left),
        Coordinate::new(down, col),
        Coordinate::new(down, right),
    ]
}

impl Day03 {}

impl Day for Day03 {
    type Input = Vec<Vec<char>>;

    // Borrowed from https://github.com/beeb/aoc-2023/blob/main/src/days/day03.rs#L147-L152
    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list1(
            line_ending,
            map(not_line_ending, |s: &str| s.chars().collect::<Vec<_>>()),
        )(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        // let mut row: usize = 0;
        // let num_rows = input.len();
        // let row_len = input.first().unwrap().len();
        let _current_num: Option<u32> = None;
        for (row_index, row) in input.iter().enumerate() {
            for (char_index, char) in row.iter().enumerate() {
                let hits = collides_with(
                    input,
                    &Coordinate {
                        row: row_index,
                        column: char_index,
                    },
                )
                .map(|t| char_at(input, &t));
                if hits.iter().any(|t| {
                    if let Some(c) = t {
                        *c != '.' && c.is_ascii_punctuation()
                    } else {
                        false
                    }
                }) {
                    if hits[3] == Some('.') {
                        // // starting a new number
                        // current_num = Some(
                        //     u32::<&str, nom::error::Error<_>>(&(*row[char_index..]))
                        //         .unwrap()
                        //         .1,
                        // );
                    }
                    // then the current char is part of a part number
                    println!(
                        "hits on {{{},{}}}({}): {hits:?}",
                        row_index, char_index, char
                    );
                }
            }
        }
        todo!();
    }
    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
