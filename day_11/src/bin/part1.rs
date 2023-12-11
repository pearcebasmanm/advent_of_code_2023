fn main() {
    let input = include_str!("../../input.txt");
    let result = part1(input);
    println!("{result}");
}

fn part1(input: &str) -> u32 {
    let mut galaxies: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(i, row)| {
            row.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(j, _)| (i, j))
        })
        .collect();

    let empty_rows: Vec<_> = (0..galaxies.iter().map(|&(x, _)| x).max().unwrap())
        .filter(|row| !galaxies.iter().any(|(x, _)| x == row))
        .collect();
    let empty_cols: Vec<_> = (0..galaxies.iter().map(|&(_, y)| y).max().unwrap())
        .filter(|col| !galaxies.iter().any(|(_, y)| y == col))
        .collect();

    for (x, y) in &mut galaxies {
        *x += empty_rows.iter().filter(|&row| row < x).count();
        *y += empty_cols.iter().filter(|&col| col < y).count();
    }

    let mut sum = 0;

    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            let (x1, y1) = galaxies[i];
            let (x2, y2) = galaxies[j];
            sum += x1.abs_diff(x2) + y1.abs_diff(y2);
        }
    }
    sum as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    const MOCK_SOLUTION: u32 = 374;

    #[test]
    fn test() {
        assert_eq!(part1(MOCK_INPUT), MOCK_SOLUTION);
    }
}
