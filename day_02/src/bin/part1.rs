use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    multi::separated_list0,
    sequence::tuple,
};

fn main() {
    let input = include_str!("../../input.txt");
    let result = part1(input);
    println!("{result}");
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(parse_line)
        .filter(|(_, batch)| is_valid(batch))
        .map(|(id, _)| id)
        .sum()
}

fn parse_line(line: &str) -> (u32, Vec<(u32, &str)>) {
    let (_, (_, id, _, sets)) = tuple::<_, _, (), _>((
        tag("Game "),
        digit1,
        tag(": "),
        separated_list0(
            alt((tag("; "), tag(", "))),
            tuple((digit1, tag(" "), alpha1)),
        ),
    ))(line)
    .unwrap();

    let id = id.parse().unwrap();
    let sets = sets
        .into_iter()
        .map(|(amount, _, color)| (amount.parse().unwrap(), color))
        .collect();
    (id, sets)
}

fn is_valid(sets: &[(u32, &str)]) -> bool {
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
