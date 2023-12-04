use std::{
    collections::{HashSet, VecDeque},
    hash::Hasher,
    ops::RangeInclusive,
};

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1, u32},
    combinator::map,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use crate::days::Day;

#[derive(Debug, Eq)]
pub struct Card {
    index: u32,
    winners: Vec<u32>,
    contestants: Vec<u32>,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Card {
    fn score(num_matches: usize) -> u32 {
        if num_matches == 0 {
            0
        } else {
            let mut score = 1;
            for _ in 0..(num_matches - 1) {
                score = score * 2
            }
            score
        }
    }

    fn num_matches(&self) -> usize {
        self.contestants
            .iter()
            .filter(|t| self.winners.contains(t))
            .count()
    }

    fn score_p1(&self) -> u32 {
        Self::score(self.num_matches())
    }

    fn win_indexes(&self) -> Option<RangeInclusive<usize>> {
        if self.num_matches() == 0 {
            None
        } else {
            Some(((self.index as usize) + 1..=(self.index as usize + self.num_matches())))
        }
    }
}

pub struct Day04;

impl Day04 {
    fn parse_single_card(input: &str) -> IResult<&str, Card> {
        map(
            tuple((
                tag("Card"),
                space1,
                u32,
                tag(":"),
                space1,
                separated_list1(space1, u32),
                tag(" |"),
                space1,
                separated_list1(space1, u32),
            )),
            |(_, _, index, _, _, winners, _, _, contestants)| Card {
                index,
                winners,
                contestants,
            },
        )(input)
    }
}

impl Day for Day04 {
    type Input = Vec<Card>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list1(line_ending, Self::parse_single_card)(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.iter().map(|g| g.score_p1() as usize).sum()
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut cards_won = input.len();
        let mut cards_to_check: Vec<usize> = Default::default();
        // push all the cards in first
        for card in 1..=input.len() {
            cards_to_check.push(card); // push the first card
        }
        // println!("array: {:?}", cards_to_check);
        while let Some(next_idx) = cards_to_check.pop() {
            let card = input.get(next_idx - 1).unwrap();
            if let Some(wins) = card.win_indexes() {
                // let score = card.num_matches();
                // println!(
                //     "Card {} wins {} cards, adding {:?}",
                //     card.index, score, wins
                // );
                // cards_won = cards_won + score; // increment the count
                for index in wins {
                    cards_won = cards_won + 1;
                    cards_to_check.push(index)
                }
            }
            // println!("array: {:?}", cards_to_check);
            cards_to_check.sort()
        }
        cards_won
    }
}
