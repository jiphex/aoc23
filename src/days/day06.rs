use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1, u64},
    combinator::map,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use crate::days::Day;

pub struct Day06;

#[derive(Debug, Default)]
pub struct Race {
    time_ms: u64,
    record_distance_mm: u64,
}

impl Race {
    fn travel_distance(&self, hold_time_ms: u64) -> u64 {
        match hold_time_ms {
            press_time if press_time >= self.time_ms => 0, // held beyond/until end of race
            press_time if press_time == 0 => 0,            // not pressed, no travel
            speed => {
                let travel_time = self.time_ms - hold_time_ms;
                speed * travel_time
            }
        }
    }
}

mod test {
    use super::Race;

    #[test]
    fn test_hold_time_example() {
        let race1 = Race {
            time_ms: 7,
            record_distance_mm: 9,
        };
        assert_eq!(race1.travel_distance(0), 0);
        assert_eq!(race1.travel_distance(1), 6);
        assert_eq!(race1.travel_distance(2), 10);
        assert_eq!(race1.travel_distance(3), 12);
        assert_eq!(race1.travel_distance(4), 12);
        assert_eq!(race1.travel_distance(5), 10);
        assert_eq!(race1.travel_distance(6), 6);
        assert_eq!(race1.travel_distance(7), 0);
        assert_eq!(race1.travel_distance(u64::MAX), 0);
    }
}

fn combine_digits(lhs: u64, rhs: u64) -> Option<u64> {
    let xstr = format!("{lhs}{rhs}");
    println!("{lhs}+{rhs} combined to {xstr}");
    xstr.parse().ok()
}

impl Day for Day06 {
    type Input = Vec<Race>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        map(
            tuple((
                tag("Time:"),
                space1,
                separated_list1(space1, u64),
                line_ending,
                tag("Distance:"),
                space1,
                separated_list1(space1, u64),
            )),
            |(_, _, times, _, _, _, distances)| {
                let t: Vec<Race> = times
                    .iter()
                    .enumerate()
                    .map(|t| Race {
                        time_ms: *t.1,
                        record_distance_mm: *distances.get(t.0).unwrap(),
                    })
                    .collect();
                t
            },
        )(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let race_ways_to_win = input.iter().map(|race| {
            (1..race.time_ms)
                .map(|press_time| race.travel_distance(press_time))
                .filter(|distance| distance > &race.record_distance_mm)
                .count()
        });
        race_ways_to_win.fold(1, |acc, x| acc * x)
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let p2_race = input
            .iter()
            .fold(None, |acc: Option<Race>, r| {
                if let Some(acc) = acc {
                    Some(Race {
                        record_distance_mm: combine_digits(
                            acc.record_distance_mm,
                            r.record_distance_mm,
                        )
                        .unwrap(),
                        time_ms: combine_digits(acc.time_ms, r.time_ms).unwrap(),
                    })
                } else {
                    Some(Race {
                        time_ms: r.time_ms,
                        record_distance_mm: r.record_distance_mm,
                    })
                }
            })
            .unwrap();
        println!("{p2_race:?}");

        (1..p2_race.time_ms)
            .map(|press_time| p2_race.travel_distance(press_time))
            .filter(|distance| distance > &p2_race.record_distance_mm)
            .count()
    }
}
