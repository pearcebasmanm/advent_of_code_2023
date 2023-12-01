fn main() {
    let input = include_str!("../../input.txt");
    let result = part1(input);
    println!("{result}");
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let first_digit = line.chars().find(|c| c.is_ascii_digit()).unwrap();
            let last_digit = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
            format!("{first_digit}{last_digit}").parse::<u32>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const MOCK_SOLUTION: u32 = 142;

    #[test]
    fn test() {
        assert_eq!(part1(MOCK_INPUT), MOCK_SOLUTION);
    }
}
