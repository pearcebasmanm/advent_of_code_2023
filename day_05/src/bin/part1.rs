use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{self, char, newline},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    AsChar, IResult,
};

fn main() {
    let input = include_str!("../../input.txt");
    let result = part1(input);
    println!("{result}");
}

fn part1(input: &str) -> i64 {
    let (seeds, conversions) = parse(input).unwrap().1;
    let conversion_steps: Vec<_> = conversions
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
    seeds
        .into_iter()
        .map(|mut seed| {
            for maps in &conversion_steps {
                seed += maps
                    .iter()
                    .find(|(range, _)| range.contains(&seed))
                    .map(|&(_, offset)| offset)
                    .unwrap_or(0);
            }
            seed
        })
        .min()
        .unwrap()
}

fn parse(input: &str) -> IResult<&str, (Vec<i64>, Vec<Vec<(i64, i64, i64)>>)> {
    separated_pair(
        preceded(tag("seeds: "), separated_list1(tag(" "), complete::i64)),
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
        assert_eq!(part1(MOCK_INPUT), MOCK_SOLUTION);
    }
}
