use std::{collections::HashSet, ops::Deref};

const NUM_ROWS: usize = 192;

fn get_intersection(line: &str) -> Option<Vec<u64>> {
    let (_, numbers) = line.split_once(':')?;
    let (win, have) = numbers.split_once('|')?;

    let have_set = have
        .split(' ')
        .filter_map(|s| s.parse().ok())
        .collect::<HashSet<u64>>();

    let win_set = win
        .split(' ')
        .filter_map(|s| s.parse().ok())
        .collect::<HashSet<u64>>();

    have_set
        .intersection(&win_set)
        .copied()
        .collect::<Vec<u64>>()
        .into()
}

fn get_score(line: &str) -> Option<u64> {
    let wins = get_intersection(line)?.len();
    if wins == 0 {
        return None;
    }
    2_u64.pow(wins as u32 - 1u32).into()
}

fn get_cards_rec(lines: &[String], row: usize, mut cards: [u64; NUM_ROWS]) -> [u64; NUM_ROWS] {
    if row >= lines.len() {
        return cards;
    }
    match get_intersection(&lines[row]) {
        Some(v) if !v.is_empty() => v
            .iter()
            .enumerate()
            .for_each(|(offset, _)| cards[row + offset + 1] += cards[row]),
        _ => (),
    };
    get_cards_rec(lines, row + 1, cards)
}

fn main() {
    let lines = aoc23::read_lines("../inputs/day04.txt");
    let part_1: u64 = lines.iter().map(Deref::deref).filter_map(get_score).sum();
    println!("part_1 {part_1}");

    let part_2: u64 = get_cards_rec(&lines, 0, [1; NUM_ROWS]).iter().sum();
    println!("part_2 {part_2}");
}
