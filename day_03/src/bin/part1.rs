use std::{ops::RangeInclusive, str::FromStr};

fn main() {
    let input = include_str!("../../input.txt");
    let result = part1(input);
    println!("{result}");
}

fn part1(input: &str) -> u32 {
    let schematic: Schematic = input.parse().unwrap();
    schematic.part_number_sum()
}

struct Schematic {
    numbers: Vec<Vec<Number>>,
    symbols: Vec<Vec<usize>>,
}

struct Number {
    value: u32,
    range: RangeInclusive<usize>,
}

impl Schematic {
    fn part_number_sum(mut self) -> u32 {
        let mut sum = 0;
        for (i, row) in self.symbols.iter().enumerate() {
            for j in row {
                for row_offset in [-1, 0, 1] {
                    let row_index = i as i32 + row_offset;
                    if !(0..(self.symbols.len() as i32)).contains(&row_index) {
                        continue;
                    }
                    self.numbers[row_index as usize].retain(|number| {
                        if number.range.contains(j) {
                            sum += number.value;
                            false
                        } else {
                            true
                        }
                    })
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
        let symbols = s
            .lines()
            .map(|line| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, c)| !c.is_ascii_digit() && c != '.')
                    .map(|(i, _)| i)
                    .collect()
            })
            .collect();
        Ok(Schematic { numbers, symbols })
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

    const MOCK_SOLUTION: u32 = 4361;

    #[test]
    fn test() {
        assert_eq!(part1(MOCK_INPUT), MOCK_SOLUTION);
    }
}
