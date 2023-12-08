use std::{collections::BTreeMap, iter::repeat};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char},
    sequence::{delimited, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("../../input.txt");
    let result = part1(input);
    println!("{result}");
}

fn part1(input: &str) -> usize {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();

    let map: BTreeMap<_, _> = nodes
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect();

    let mut element = "AAA";

    instructions
        .chars()
        .cycle()
        .position(|instruction| {
            element = match instruction {
                'L' => map.get(element).unwrap().0,
                'R' => map.get(element).unwrap().1,
                _ => unreachable!(),
            };
            element == "ZZZ"
        })
        .unwrap()
        + 1
}

fn parse_line(line: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(
        alpha1,
        tag(" = "),
        delimited(
            char('('),
            separated_pair(alpha1, tag(", "), alpha1),
            char(')'),
        ),
    )(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        const MOCK_INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        const MOCK_SOLUTION: usize = 2;

        assert_eq!(part1(MOCK_INPUT), MOCK_SOLUTION);
    }

    #[test]
    fn test2() {
        const MOCK_INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        const MOCK_SOLUTION: usize = 6;

        assert_eq!(part1(MOCK_INPUT), MOCK_SOLUTION);
    }
}
