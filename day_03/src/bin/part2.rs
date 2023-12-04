use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("../../input.txt");
    let result = part2(input);
    println!("{result}");
}

fn part2(input: &str) -> u32 {
    let numbers: Vec<Vec<Number>> = input
        .lines()
        .map(|line| {
            let mut row = Vec::new();
            let mut start = 0;
            let mut num = None;
            for (i, ch) in line.chars().enumerate() {
                num = match (ch.is_ascii_digit(), num) {
                    (true, Some(num)) => Some(num * 10 + ch.to_digit(10).unwrap()),
                    (true, None) => {
                        start = i;
                        Some(ch.to_digit(10).unwrap())
                    }
                    (false, Some(value)) => {
                        let range = start.saturating_sub(1)..=i;
                        row.push(Number { value, range });
                        None
                    }
                    (false, None) => None,
                }
            }
            if let Some(value) = num {
                let range = start.saturating_sub(1)..=(line.len() - 1);
                row.push(Number { value, range });
            }
            row
        })
        .collect();
    let valid_row = |row: &i32| (0..(numbers.len() as i32)).contains(row);
    input
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '*')
                .map(|(i, _)| i)
        })
        .enumerate()
        .map(|(i, row)| {
            row.into_iter()
                .map(|gear| {
                    [-1, 0, 1]
                        .into_iter()
                        .map(|row_offset| i as i32 + row_offset)
                        .filter(valid_row)
                        .flat_map(|row_index| {
                            numbers[row_index as usize]
                                .iter()
                                .filter(|number| number.range.contains(&gear))
                                .map(|number| number.value)
                        })
                        .collect::<Vec<_>>()
                })
                .filter(|parts| parts.len() == 2)
                .map(|parts| parts[0] * parts[1])
                .sum::<u32>()
        })
        .sum()
}

struct Number {
    value: u32,
    range: RangeInclusive<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    const MOCK_SOLUTION: u32 = 467835;

    #[test]
    fn test() {
        assert_eq!(part2(MOCK_INPUT), MOCK_SOLUTION);
    }
}
