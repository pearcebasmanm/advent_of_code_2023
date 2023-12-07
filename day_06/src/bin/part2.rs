fn main() {
    let input = include_str!("../../input.txt");
    let result = part2(input);
    println!("{result}");
}

fn part2(input: &str) -> u64 {
    let [time, distance] = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap()
        })
        .collect::<Vec<u64>>()
        .try_into()
        .unwrap();
    (0..=time)
        .filter(|hold| (time - hold) * hold > distance)
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    const MOCK_SOLUTION: u64 = 71503;

    #[test]
    fn test() {
        assert_eq!(part2(MOCK_INPUT), MOCK_SOLUTION);
    }
}
