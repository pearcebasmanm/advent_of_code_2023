fn main() {
    let input = include_str!("../../input.txt");
    let result = part2(input, 1_000_000);
    println!("{result}");
}

fn part2(input: &str, factor: usize) -> usize {
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
        *x += (factor - 1) * empty_rows.iter().filter(|&row| row < x).count();
        *y += (factor - 1) * empty_cols.iter().filter(|&col| col < y).count();
    }

    let mut sum = 0;

    for i in 0..(galaxies.len() - 1) {
        for j in (i + 1)..galaxies.len() {
            let (x1, y1) = galaxies[i];
            let (x2, y2) = galaxies[j];
            sum += x1.abs_diff(x2) + y1.abs_diff(y2);
        }
    }
    sum
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

    #[test]
    fn test() {
        assert_eq!(part2(MOCK_INPUT, 10), 1030);
        assert_eq!(part2(MOCK_INPUT, 100), 8410);
    }
}
