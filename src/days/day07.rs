use nom::{
    branch::alt,
    character::{complete::alpha1, is_digit},
    combinator::map_res,
    Err, IResult,
};

use crate::days::Day;

pub struct Day07;

pub enum Card {
    King,
    Queen,
    Jack,
    Ace,
    Number(u8),
}

impl Day07 {
    fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
        map_res(alpha1, |t: &str| {
            t.chars()
                .map(|card| {
                    return match t {
                        // card if card == 'K' => Ok(Card::King),
                        _ => Err("foo"),
                    };
                })
                .collect()
        })(input)
    }
}

impl Day for Day07 {
    type Input = String;

    fn parse(_input: &str) -> IResult<&str, Self::Input> {
        unimplemented!("parser")
    }

    type Output1 = usize;

    fn part_1(_input: &Self::Input) -> Self::Output1 {
        unimplemented!("part_1")
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
