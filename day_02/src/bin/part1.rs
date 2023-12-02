use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1},
    multi::separated_list0,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../../input.txt");
    let result = part1(input);
    println!("{result}");
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .filter(|(_, batch)| is_valid(batch))
        .map(|(id, _)| id)
        .sum()
}

type Cubes<'a> = (u32, &'a str);

fn parse_line(line: &str) -> IResult<&str, (u32, Vec<Cubes>)> {
    tuple((
        delimited(tag("Game "), complete::u32, tag(": ")),
        separated_list0(
            alt((tag("; "), tag(", "))),
            separated_pair(complete::u32, tag(" "), alpha1),
        ),
    ))(line)
}

fn is_valid(sets: &[Cubes]) -> bool {
    sets.iter()
        .all(|set| matches!(set, (1..=12, "red") | (1..=13, "green") | (1..=14, "blue")))
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    const MOCK_SOLUTION: u32 = 8;

    #[test]
    fn test() {
        assert_eq!(part1(MOCK_INPUT), MOCK_SOLUTION);
    }
}
