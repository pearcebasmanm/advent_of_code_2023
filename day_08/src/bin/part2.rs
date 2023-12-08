use std::collections::BTreeMap;

use num::integer::lcm;

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, char},
    sequence::{delimited, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("../../input.txt");
    let result = part2(input);
    println!("{result}");
}

fn part2(input: &str) -> usize {
    let (instructions, nodes) = input.split_once("\n\n").unwrap();

    let map: BTreeMap<_, _> = nodes
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect();

    map.keys()
        .filter(|key| key.ends_with('A'))
        .copied()
        .map(|mut element| {
            instructions
                .chars()
                .cycle()
                .position(|instruction| {
                    element = match instruction {
                        'L' => map.get(element).unwrap().0,
                        'R' => map.get(element).unwrap().1,
                        _ => unreachable!(),
                    };
                    element.ends_with('Z')
                })
                .unwrap()
                + 1
        })
        .reduce(lcm)
        .unwrap()
}

fn parse_line(line: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(
        alphanumeric1,
        tag(" = "),
        delimited(
            char('('),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            char(')'),
        ),
    )(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_INPUT: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    const MOCK_SOLUTION: usize = 6;

    #[test]
    fn test() {
        assert_eq!(part2(MOCK_INPUT), MOCK_SOLUTION);
    }
}
