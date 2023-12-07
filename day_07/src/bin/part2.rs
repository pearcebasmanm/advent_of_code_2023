#![allow(clippy::enum_variant_names)]

use std::{collections::BTreeSet, str::FromStr};

fn main() {
    let input = include_str!("../../input.txt");
    let result = part2(input);
    println!("{result}");
}

fn part2(input: &str) -> u32 {
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
            .filter(|&card| card != &Card::Joker)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|card| self.0.iter().filter(|c| c == &card).count())
            .collect::<Vec<usize>>();
        breakdown.sort();
        breakdown.reverse();

        let jokers = self.0.iter().filter(|&card| card == &Card::Joker).count();

        match breakdown[..] {
            [] | [_] => Kind::FiveOfAKind,
            [n, ..] if n + jokers == 4 => Kind::FourOfAKind,
            [n, 2] if n + jokers == 3 => Kind::FullHouse,
            [n, 1, 1] if n + jokers == 3 => Kind::ThreeOfAKind,
            [2, 2, 1] => Kind::TwoPair,
            [n, ..] if n + jokers == 2 => Kind::OnePair,
            _ => Kind::HighCard,
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
            .and_then(|vec| vec.try_into().ok())
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
            'J' => Card::Joker,
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
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
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

    const MOCK_SOLUTION: u32 = 5905;

    #[test]
    fn test() {
        assert_eq!(part2(MOCK_INPUT), MOCK_SOLUTION);
    }
}
