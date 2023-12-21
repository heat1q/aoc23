use itertools::{FoldWhile, Itertools};
use std::collections::HashMap;

fn walk_one<'a>(
    map: &'a HashMap<&'a str, (&'a str, &'a str)>,
    cur: &'a str,
    instruction: u8,
) -> &'a str {
    let next = map.get(cur).unwrap();
    match instruction {
        b'L' => next.0,
        b'R' => next.1,
        _ => unreachable!(),
    }
}

fn walk(
    map: &HashMap<&str, (&str, &str)>,
    instructions: &[u8],
    start: &str,
    endf: impl Fn(&str) -> bool,
) -> u64 {
    let (num_steps, _) = instructions
        .iter()
        .cycle()
        .fold_while((0, start), |(s, cur), n| {
            let next = walk_one(map, cur, *n);
            let s = s + 1;
            if endf(next) {
                FoldWhile::Done((s, next))
            } else {
                FoldWhile::Continue((s, next))
            }
        })
        .into_inner();
    num_steps
}

fn parse_instructions(lines: &[String]) -> &[u8] {
    lines.iter().next().unwrap().trim().as_bytes()
}

fn parse_map(lines: &[String]) -> HashMap<&str, (&str, &str)> {
    lines
        .iter()
        .skip(2)
        .filter_map(|l| l.split_once('='))
        .filter_map(|(node, dir)| {
            let mut dir = dir.split(['(', ')', ',', ' ']).filter(|c| !c.is_empty());
            let left = dir.next()?;
            let right = dir.next()?;
            Some((node.trim(), (left, right)))
        })
        .collect()
}

fn part1(lines: &[String]) -> u64 {
    let instructions = parse_instructions(lines);
    let map = parse_map(lines);
    walk(&map, instructions, "AAA", |n| n == "ZZZ")
}

fn part2(lines: &[String]) -> u64 {
    let instructions = parse_instructions(lines);
    let map = parse_map(lines);
    map.keys()
        .filter(|k| k.ends_with('A'))
        .map(|n| walk(&map, instructions, n, |n| n.ends_with('Z')))
        .fold(1, num::integer::lcm)
}

fn main() {
    let lines = aoc23::read_lines("../inputs/day08.txt");

    let part1 = part1(&lines);
    println!("part_1 {part1}");

    let part2 = part2(&lines);
    println!("part_2 {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_directions() {
        let lines = [
            "LR",
            "",
            "11A = (11B, XXX)",
            "11B = (XXX, 11Z)",
            "11Z = (11B, XXX)",
            "22A = (22B, XXX)",
            "22B = (22C, 22C)",
            "22C = (22Z, 22Z)",
            "22Z = (22B, 22B)",
            "XXX = (XXX, XXX)",
        ]
        .map(String::from);

        assert_eq!(6, part2(&lines));
    }
}
