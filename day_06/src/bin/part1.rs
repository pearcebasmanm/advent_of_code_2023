use nom::{
    bytes::complete::tag,
    character::complete::{self, multispace1, newline},
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("../../input.txt");
    let result = part1(input);
    println!("{result}");
}

fn part1(input: &str) -> u32 {
    parse(input)
        .unwrap()
        .1
        .into_iter()
        .map(process_race)
        .product()
}

fn parse(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (remainder, (times, distances)) = separated_pair(
        preceded(
            pair(tag("Time:"), multispace1),
            separated_list1(multispace1, complete::u32),
        ),
        newline,
        preceded(
            pair(tag("Distance:"), multispace1),
            separated_list1(multispace1, complete::u32),
        ),
    )(input)?;
    let races = times.into_iter().zip(distances).collect();
    Ok((remainder, races))
}

fn process_race(input: (u32, u32)) -> u32 {
    let (time, distance) = input;
    (0..=time)
        .filter(|hold| (time - hold) * hold > distance)
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    const MOCK_SOLUTION: u32 = 288;

    #[test]
    fn test() {
        assert_eq!(part1(MOCK_INPUT), MOCK_SOLUTION);
    }
}
