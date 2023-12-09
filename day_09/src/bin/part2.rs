fn main() {
    let input = include_str!("../../input.txt");
    let result = part2(input);
    println!("{result}");
}

fn part2(input: &str) -> i32 {
    input.lines().map(parse_line).map(process_line).sum()
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split(' ').flat_map(str::parse).collect()
}

fn process_line(nums: Vec<i32>) -> i32 {
    let mut derivatives = vec![nums];
    loop {
        let next: Vec<_> = derivatives
            .last()
            .unwrap()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect();
        if next.iter().all(|&num| num == 0) {
            break;
        }
        derivatives.push(next);
    }

    derivatives
        .into_iter()
        .rev()
        .map(|nums| *nums.first().unwrap())
        .reduce(|acc, new| new - acc)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    const MOCK_SOLUTION: i32 = 2;

    #[test]
    fn test() {
        assert_eq!(part2(MOCK_INPUT), MOCK_SOLUTION);
    }
}
