use itertools::Itertools;
use std::{cmp::Ordering, iter};

fn map_score(card: u8) -> u64 {
    match card {
        b'2'..=b'9' => (card - b'0') as u64,
        b'T' => 10,
        b'J' => 11,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        _ => 0,
    }
}

#[derive(Debug, PartialEq)]
struct Hand<'a> {
    hand: &'a str,
    score: u64,
}

impl<'a> Hand<'a> {
    fn from_input(hand: &'a str) -> Self {
        let freq: Vec<u8> = hand
            .as_bytes()
            .iter()
            .fold([0; 127], |mut acc: [u8; 127], n| {
                acc[*n as usize] += 1;
                acc
            })
            .into_iter()
            .enumerate()
            .filter_map(|(_card, freq)| if freq > 0 { Some(freq) } else { None })
            .sorted()
            .collect();

        let score = match freq[..] {
            [5] => 7,
            [1, 4] => 6,
            [2, 3] => 5,
            [1, 1, 3] => 4,
            [1, 2, 2] => 3,
            [1, 1, 1, 2] => 2,
            _ => 1,
        };

        Self { hand, score }
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.score == other.score {
            return if let Some((a, b)) =
                iter::zip(self.hand.as_bytes().iter(), other.hand.as_bytes().iter())
                    .find(|(a, b)| a != b)
            {
                map_score(*a).partial_cmp(&map_score(*b))
            } else {
                Some(Ordering::Equal)
            };
        }
        self.score.partial_cmp(&other.score)
    }
}

fn get_winnings(lines: &[String]) -> u64 {
    lines
        .iter()
        .filter_map(|l| {
            let (hand, bid) = l.split_once(' ')?;
            let hand = Hand::from_input(hand);
            let bid: u64 = bid.parse().ok()?;
            Some((hand, bid))
        })
        .sorted_by(|(l, _), (r, _)| l.partial_cmp(r).unwrap())
        .enumerate()
        .fold(0, |acc, (i, (_, b))| acc + (i + 1) as u64 * b)
}

fn main() {
    let lines = aoc23::read_lines("../inputs/day07.txt");
    let winnings = get_winnings(&lines);
    println!("part_1/part_2 {winnings}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand() {
        let hand = Hand::from_input("32T3K");
        assert_eq!(hand.score, 2);
        let fullhouse = Hand::from_input("32323");
        assert_eq!(fullhouse.score, 5);
    }

    #[test]
    fn cmp() {
        let l = Hand::from_input("2AAAA");
        let r = Hand::from_input("33332");
        assert_eq!(Ordering::Less, l.partial_cmp(&r).unwrap())
    }

    #[test]
    fn part1() {
        let lines = [
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .map(String::from);
        let winnings = get_winnings(&lines);
        assert_eq!(6440, winnings);
    }
}
