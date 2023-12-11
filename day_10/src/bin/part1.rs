use std::{ops::Neg, str::FromStr};

fn main() {
    let input = include_str!("../../input.txt");
    let result = part1(input);
    println!("{result}");
}

fn part1(input: &str) -> u32 {
    let table: Table = input.parse().unwrap();
    table.solve()
}

struct Table(Vec<Vec<char>>);

impl Table {
    fn solve(self) -> u32 {
        let (x, y) = self.start_coords();
        let mut paths = self.neighbors(x, y);
        assert_eq!(paths.len(), 2);
        (2..)
            .find(|i| {
                println!("{i} {paths:?}");
                for path in &mut paths {
                    let (prev_direction, (x, y)) = *path;
                    let mut new_neighbor: Vec<_> = self.neighbors(x, y);
                    new_neighbor.retain(|&(direction, _)| direction != -prev_direction);
                    assert_eq!(new_neighbor.len(), 1);
                    *path = new_neighbor.pop().unwrap();
                }
                dbg!(paths[0].1 == paths[1].1)
            })
            .unwrap()
    }
    fn start_coords(&self) -> (usize, usize) {
        self.0
            .iter()
            .enumerate()
            .find_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .find(|&(_, &c)| c == 'S')
                    .map(|(j, _)| (i, j))
            })
            .unwrap()
    }
    fn neighbors(&self, x: usize, y: usize) -> Vec<(Direction, (usize, usize))> {
        let mut neighbors = Vec::new();
        if x > 0
            && Direction::Up.chars().contains(&self.0[x][y])
            && Direction::Up.neg().chars().contains(&self.0[x - 1][y])
        {
            neighbors.push((Direction::Up, (x - 1, y)));
        }
        if x < self.0.len() - 1
            && Direction::Down.chars().contains(&self.0[x][y])
            && Direction::Down.neg().chars().contains(&self.0[x + 1][y])
        {
            neighbors.push((Direction::Down, (x + 1, y)));
        }
        if y > 0
            && Direction::Left.chars().contains(&self.0[x][y])
            && Direction::Left.neg().chars().contains(&self.0[x][y - 1])
        {
            neighbors.push((Direction::Left, (x, y - 1)));
        }
        if y < self.0[x].len() - 1
            && Direction::Right.chars().contains(&self.0[x][y])
            && Direction::Right.neg().chars().contains(&self.0[x][y + 1])
        {
            neighbors.push((Direction::Right, (x, y + 1)));
        }
        neighbors
    }
}

impl FromStr for Table {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let table: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        Ok(Table(table))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn chars(&self) -> [char; 4] {
        match self {
            Self::Up => ['S', '|', 'J', 'L'],
            Self::Down => ['S', '|', '7', 'F'],
            Self::Left => ['S', '-', 'J', '7'],
            Self::Right => ['S', '-', 'L', 'F'],
        }
    }
}

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_INPUT: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    const MOCK_SOLUTION: u32 = 4;

    #[test]
    fn test() {
        assert_eq!(part1(MOCK_INPUT), MOCK_SOLUTION);
    }
}
