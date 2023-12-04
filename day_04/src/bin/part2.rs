use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{self, multispace1},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../../input.txt");
    let result = part2(input);
    println!("{result}");
}

fn part2(input: &str) -> u32 {
    let mut map = BTreeMap::new();
    input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .map(|processed| process(&mut map, processed))
        .sum()
}

fn parse_line(line: &str) -> IResult<&str, (u32, Vec<u32>, Vec<u32>)> {
    let (remainder, (card, (winning_numbers, numbers))) = separated_pair(
        preceded(tuple((tag("Card"), multispace1)), complete::u32),
        tuple((tag(":"), multispace1)),
        separated_pair(
            separated_list1(multispace1, complete::u32),
            tuple((multispace1, tag("|"), multispace1)),
            separated_list1(multispace1, complete::u32),
        ),
    )(line)?;
    Ok((remainder, (card, winning_numbers, numbers)))
}

fn process(map: &mut BTreeMap<u32, u32>, input: (u32, Vec<u32>, Vec<u32>)) -> u32 {
    let (card, winning_numbers, numbers) = input;

    let amount_of_current = 1 + map.remove(&card).unwrap_or(0);

    let matches = numbers
        .into_iter()
        .filter(|number| winning_numbers.contains(number))
        .count() as u32;

    for card in (1..=matches).map(|i| card + i) {
        *map.entry(card).or_insert(0) += amount_of_current;
    }

    amount_of_current
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    const MOCK_SOLUTION: u32 = 30;

    #[test]
    fn test() {
        assert_eq!(part2(MOCK_INPUT), MOCK_SOLUTION);
    }
}
