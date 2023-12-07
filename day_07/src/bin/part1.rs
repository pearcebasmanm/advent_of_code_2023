#![allow(clippy::enum_variant_names)]

use std::{collections::BTreeSet, str::FromStr};

fn main() {
    let input = include_str!("../../input.txt");
    let result = part1(input);
    println!("{result}");
}

fn part1(input: &str) -> u32 {
    let mut games: Vec<_> = input.lines().map(parse_line).collect();
    games.sort();
    games
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as u32 + 1) * bid)
        .sum()
}

fn parse_line(line: &str) -> (Hand, u32) {
    let (hand, bid) = line.split_once(' ').unwrap();
    (hand.parse().unwrap(), bid.parse().unwrap())
}

impl Hand {
    fn kind(&self) -> Kind {
        let mut breakdown = self
            .0
            .iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|card| self.0.iter().filter(|c| c == &card).count())
            .collect::<Vec<usize>>();
        breakdown.sort();
        breakdown.reverse();

        match breakdown[..] {
            [5] => Kind::FiveOfAKind,
            [4, 1] => Kind::FourOfAKind,
            [3, 2] => Kind::FullHouse,
            [3, 1, 1] => Kind::ThreeOfAKind,
            [2, 2, 1] => Kind::TwoPair,
            [2, 1, 1, 1] => Kind::OnePair,
            [1, 1, 1, 1, 1] => Kind::HighCard,
            _ => unreachable!(),
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .chars()
            .map(|c| c.try_into().ok())
            .collect::<Option<Vec<_>>>()
            .ok_or(())?
            .try_into()
            .ok()
            .ok_or(())?;
        Ok(Hand(cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.kind(), &self.0).cmp(&(other.kind(), &other.0)))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let card = match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => return Err(()),
        };
        Ok(card)
    }
}

#[derive(PartialEq, Eq)]
struct Hand([Card; 5]);

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    const MOCK_SOLUTION: u32 = 6440;

    #[test]
    fn test() {
        assert_eq!(part1(MOCK_INPUT), MOCK_SOLUTION);
    }
}
