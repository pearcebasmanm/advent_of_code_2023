#![allow(clippy::single_range_in_vec_init)]

use std::ops::RangeInclusive;

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

    let mut ranges: Vec<_> = seed_groups
        .into_iter()
        .map(|(start, length)| start..=start + length - 1)
        .collect();

    for conversion in &conversions {
        let mut i = 0;
        let mut next_ranges = Vec::new();
        println!("Next conversion");
        loop {
            if ranges.iter().all(|range| {
                conversion
                    .iter()
                    .all(|(source, _)| overlap(range.clone(), source.clone()).0.is_none())
            }) || ranges.is_empty()
            {
                break;
            }

            i += 1;
            if i > 100 {
                break;
            }

            println!(
                "RangeInclusives: {}",
                ranges
                    .iter()
                    .map(|range| format!("{}..{}", range.start(), range.end()))
                    .collect::<Vec<String>>()
                    .join(" ")
            );
            for (source_range, offset) in conversion {
                if ranges.is_empty() {
                    break;
                }
                let this_range = ranges[0].clone();
                print!("Checking {this_range:?} & {source_range:?} | ");
                let (overlap, remainder) = overlap(this_range, source_range.clone());
                if let Some(range) = overlap {
                    if !range.is_empty() {
                        next_ranges.push(range.start() + offset..range.end() + offset);
                    }
                    print!("Overlap: {}..{}, ", range.start(), range.end());
                }
                for range in remainder {
                    print!("Remainder: {range:?}, ");
                    ranges.push(range);
                }
                println!();
                ranges.remove(0);
            }
        }
        ranges.append(&mut next_ranges);
    }
    ranges.into_iter().map(|range| range.start()).min().unwrap()
}

fn overlap(
    this: RangeInclusive<i64>,
    other: RangeInclusive<i64>,
) -> (Option<RangeInclusive<i64>>, Vec<RangeInclusive<i64>>) {
    // use std::ops::Ordering::*;
    // match (
    //     this.start().cmp(&other.start()),
    //     this.start().cmp(&other.end()),
    //     this.end().cmp(&other.start()),
    //     this.end().cmp(&other.end()),
    // ) {
    //     (Less | Equal, Less, )
    // }

    // // no overlap
    if this.end() < other.start() || other.end() < this.start() {
        (None, vec![this])

    // entirely contained
    } else if other.start() <= this.start() && this.end() <= other.end() {
        (Some(this), Vec::new())

    // middle portion is contained
    } else if this.start() <= other.start() && other.end() <= this.end() {
        (
            Some(other.clone()),
            vec![*this.start()..=*other.start(), *other.end()..=*this.end()],
        )

    // rightmost portion is contained
    } else if this.start() < other.start() {
        (
            Some(*other.start()..=*this.end()),
            vec![*this.start()..=*other.start()],
        )

    // leftmost potion is contained
    } else if other.end() < this.end() && this.end() != other.start() {
        println!("there");
        (
            Some(*this.start()..*other.end()),
            vec![other.end()..this.end()],
        )

    // I think I covered my bases, so never
    } else {
        panic!(
            "{this:?} {other:?} | {}",
            other.start() <= this.start() && this.end() <= other.end()
        );
    }
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
