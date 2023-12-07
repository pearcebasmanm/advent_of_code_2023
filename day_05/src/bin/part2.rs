#![allow(clippy::single_range_in_vec_init)]

use std::{cmp::Ordering, process::exit};

use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{self, char, newline},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    AsChar, IResult,
};

fn main() {
    let input = include_str!("../../input.txt");
    let result = part2(input);
    println!("{result}");
}

fn part2(input: &str) -> i64 {
    let (seed_groups, conversions) = parse(input).unwrap().1;
    let conversions: Vec<_> = conversions
        .into_iter()
        .map(|ranges| {
            ranges
                .into_iter()
                .map(|(destination_start, source_start, length)| {
                    (
                        source_start..(source_start + length),
                        destination_start - source_start,
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect();
    seed_groups
        .into_iter()
        .map(|(start, length)| start..start + length)
        .flat_map(|range| {
            let mut ranges = vec![range];
            println!("{ranges:?}");
            for conversion in &conversions {
                let mut next_ranges = Vec::new();
                let mut i = 0;
                while !ranges.iter().all(|range| {
                    conversion
                        .iter()
                        .all(|(source, _)| source.end < range.start || range.end < source.start)
                }) {
                    println!("{ranges:?} {conversion:?}");
                    i += 1;
                    if i > 100 {
                        println!("breaking");
                        break;
                    }
                    for (source_range, offset) in conversion {
                        let this_range = ranges[0].clone();
                        let (overlap, remainder) = match (
                            this_range.start.cmp(&source_range.start),
                            this_range.end.cmp(&source_rang.end),
                        ) {
                            (
                                Ordering::Less | Ordering::Equal,
                                Ordering::Greater | Ordering::Equal,
                            ) => (Some(this_range), Vec::new()),
                            (Ordering::Less | Ordering::Equal, Ordering::Less) => (
                                Some(this_range.start..source_range.end),
                                vec![source_range.end..this_range.end],
                            ),
                            (Ordering::Greater, Ordering::Less | Ordering::Equal) => (
                                Some(this_range.start..source_range.start),
                                vec![source_range.start..this_range.end],
                            ),
                            (Ordering::Greater, Ordering::Less) => {
                                if this_range.contains(&source_range.start)
                                    && this_range.contains(&source_range.end)
                                {
                                    (
                                        Some(source_range.start..source_range.end),
                                        vec![
                                            this_range.start..source_range.start,
                                            source_range.end..this_range.end,
                                        ],
                                    )
                                } else {
                                    (None, vec![this_range])
                                }
                            }
                        };
                        if let Some(range) = overlap {
                            next_ranges.push(range.start + offset..range.end + offset);
                        }
                        for range in remainder {
                            ranges.push(range);
                        }
                        ranges.remove(0);
                    }
                }
                ranges.append(&mut next_ranges);
            }
            ranges.into_iter().map(|range| range.start)
        })
        .min()
        .unwrap()
}

enum Overlap {
    None,
    Contains,
    Contained,
}

fn overlap(left: Range<i64>, right: Range<i64>) -> Overlap {
    if left.end < right.start || right.end < left.start {
        Overlap::None
    } else if left.start <= right.start && right.start <= left.start {
        Overlap::Contains
    } else if right.start <= left.start && left.start <= right.start {
        Overlap::Contained
    } else if 
}

type Parsed = (Vec<(i64, i64)>, Vec<Vec<(i64, i64, i64)>>);

fn parse(input: &str) -> IResult<&str, Parsed> {
    separated_pair(
        preceded(
            tag("seeds: "),
            separated_list1(
                tag(" "),
                separated_pair(complete::i64, tag(" "), complete::i64),
            ),
        ),
        tag("\n\n"),
        separated_list1(
            tag("\n\n"),
            preceded(
                take_till(AsChar::is_dec_digit),
                separated_list1(
                    newline,
                    tuple((
                        complete::i64,
                        delimited(char(' '), complete::i64, char(' ')),
                        complete::i64,
                    )),
                ),
            ),
        ),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    const MOCK_SOLUTION: i64 = 35;

    #[test]
    fn test() {
        assert_eq!(part2(MOCK_INPUT), MOCK_SOLUTION);
    }
}
