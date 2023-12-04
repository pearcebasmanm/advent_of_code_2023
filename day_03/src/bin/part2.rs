use std::{ops::RangeInclusive, str::FromStr};

fn main() {
    let input = include_str!("../../input.txt");
    let result = part2(input);
    println!("{result}");
}

fn part2(input: &str) -> u32 {
    let schematic: Schematic = input.parse().unwrap();
    schematic.part_number_sum()
}

struct Schematic {
    numbers: Vec<Vec<Number>>,
    gears: Vec<Vec<usize>>,
}

struct Number {
    value: u32,
    range: RangeInclusive<usize>,
}

impl Schematic {
    fn part_number_sum(&self) -> u32 {
        let mut sum = 0;
        for (i, row) in self.gears.iter().enumerate() {
            for gear in row {
                let part_numbers: Vec<_> = [-1, 0, 1]
                    .into_iter()
                    .map(|row_offset| i as i32 + row_offset)
                    .filter(|row_index| (0..(self.gears.len() as i32)).contains(row_index))
                    .flat_map(|row_index| {
                        self.numbers[row_index as usize]
                            .iter()
                            .filter(|number| number.range.contains(gear))
                            .map(|number| number.value)
                    })
                    .collect();
                if let [first_part, second_part] = part_numbers[..] {
                    sum += first_part * second_part;
                }
            }
        }
        sum
    }
}

impl FromStr for Schematic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
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
        let gears = s
            .lines()
            .map(|line| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, c)| c == '*')
                    .map(|(i, _)| i)
                    .collect()
            })
            .collect();
        Ok(Schematic { numbers, gears })
    }
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
