use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    multi::separated_list1,
    sequence::tuple,
};

fn main() {
    let input = include_str!("../../input.txt");
    let result = part2(input);
    println!("{result}");
}

fn part2(input: &str) -> u32 {
    input.lines().map(parse_line).map(process_line).sum()
}

fn parse_line(line: &str) -> Vec<(u32, &str)> {
    tuple::<_, _, (), _>((
        tag("Game "),
        digit1,
        tag(": "),
        separated_list1(
            alt((tag("; "), tag(", "))),
            tuple((digit1, tag(" "), alpha1)),
        ),
    ))(line)
    .unwrap()
    .1
     .3
    .into_iter()
    .map(|(amount, _, color)| (amount.parse().unwrap(), color))
    .collect()
}

fn process_line(sets: Vec<(u32, &str)>) -> u32 {
    ["red", "green", "blue"]
        .iter()
        .map(|target_color| {
            sets.iter()
                .filter(|(_, color)| color == target_color)
                .map(|(amount, _)| amount)
                .max()
                .unwrap()
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    const MOCK_SOLUTION: u32 = 2286;

    #[test]
    fn test() {
        assert_eq!(part2(MOCK_INPUT), MOCK_SOLUTION);
    }
}
