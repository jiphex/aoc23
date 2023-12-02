use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::{
        complete::{alphanumeric1, line_ending},
        is_digit,
    },
    combinator::map,
    multi::separated_list1,
    IResult,
};

use crate::days::Day;

pub struct Day01;

impl Day01 {
    fn first_last_digits(input: &str) -> (u32, u32) {
        let mut output: (Option<u32>, u32) = (None, 0);
        input.chars().for_each(|c| {
            if c.is_numeric() {
                let digit = c.to_digit(10);
                if output.0.is_none() {
                    output.0 = digit;
                }
                output.1 = digit.unwrap()
            }
        });
        (output.0.unwrap(), output.1)
    }

    fn first_last_digits_words(input: &str) -> (u32, u32) {
        let mut output: (Option<u32>, u32) = (None, 0);
        input.chars().enumerate().for_each(|(pos, c)| {
            if c.is_numeric() {
                let digit = c.to_digit(10);
                if output.0.is_none() {
                    output.0 = digit;
                }
                output.1 = digit.unwrap()
            } else {
                if let Some(number) = match &input[pos..] {
                    s if s.starts_with("one") => Some(1),
                    s if s.starts_with("two") => Some(2),
                    s if s.starts_with("three") => Some(3),
                    s if s.starts_with("four") => Some(4),
                    s if s.starts_with("five") => Some(5),
                    s if s.starts_with("six") => Some(6),
                    s if s.starts_with("seven") => Some(7),
                    s if s.starts_with("eight") => Some(8),
                    s if s.starts_with("nine") => Some(9),
                    _ => None,
                } {
                    if output.0.is_none() {
                        output.0 = Some(number);
                    }
                    output.1 = number
                }
            }
        });
        (output.0.unwrap(), output.1)
    }

    fn combine_nums(input: (u32, u32)) -> u32 {
        input.0 * 10 + input.1
    }
}

impl Day for Day01 {
    type Input = Vec<String>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list1(line_ending, map(alphanumeric1, |s: &str| s.to_string()))(input)
    }

    type Output1 = u32;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .map(|s| Self::first_last_digits(s))
            .map(|s| Self::combine_nums(s))
            .sum()
    }

    type Output2 = u32;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        input
            .iter()
            .map(|s| Self::first_last_digits_words(s))
            .map(|s| Self::combine_nums(s))
            .sum()
    }
}
