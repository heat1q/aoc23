use std::{collections::HashMap, ops::Deref};

#[derive(Debug, PartialEq)]
struct Part {
    start: usize,
    end: usize,
    row: usize,
    val: Vec<u8>,
}

const POSITIONS: [[i32; 2]; 8] = [
    [1, -1],
    [1, 0],
    [1, 1],
    [0, 1],
    [0, -1],
    [-1, -1],
    [-1, 0],
    [-1, 1],
];

impl Part {
    fn id(&self) -> u64 {
        self.val
            .iter()
            .rev()
            .enumerate()
            .fold(0_u64, |acc, (i, b)| {
                acc + 10_u64.pow(i as u32) * (b - b'0') as u64
            })
    }

    fn validate(&self, lines: &[String], gear_set: &mut HashMap<(usize, usize), Vec<u64>>) -> bool {
        let max_col_i = lines.len() as i32;
        let max_row_j = lines[self.row].len() as i32;
        self.val.iter().enumerate().fold(false, |acc, (col, _)| {
            acc || POSITIONS.iter().fold(false, |acc, n| {
                let i: i32 = n[0] + self.row as i32;
                let j: i32 = n[1] + (self.start + col) as i32;
                acc || match (i, j) {
                    (0.., 0..) if i < max_col_i && j < max_row_j => {
                        let i = i as usize;
                        let j = j as usize;
                        match lines[i].as_bytes()[j] {
                            b'0'..=b'9' | b'.' => false,
                            b'*' => {
                                let products = gear_set.entry((i, j)).or_default();
                                products.push(self.id());
                                true
                            }
                            _ => true,
                        }
                    }
                    (_, _) => false,
                }
            })
        })
    }
}

fn parse_parts_line_rec(
    line: &[u8],
    row: usize,
    index: usize,
    buf: &mut Vec<Part>,
    cur: Option<Part>,
) {
    match (line.get(index), cur) {
        (Some(b @ b'0'..=b'9'), Some(mut p)) => {
            p.val.push(*b);
            parse_parts_line_rec(line, row, index + 1, buf, Some(p))
        }
        (Some(b @ b'0'..=b'9'), None) => {
            let mut val = Vec::with_capacity(3);
            val.push(*b);
            let p = Part {
                start: index,
                end: 0,
                row,
                val,
            };
            parse_parts_line_rec(line, row, index + 1, buf, Some(p))
        }
        (Some(_), Some(mut p)) => {
            p.end = index;
            buf.push(p);
            parse_parts_line_rec(line, row, index + 1, buf, None)
        }
        (Some(_), None) => parse_parts_line_rec(line, row, index + 1, buf, None),
        (None, Some(mut p)) => {
            p.end = index;
            buf.push(p);
        }
        (None, None) => (),
    }
}

fn parse_parts_line((row, line): (usize, &str)) -> Vec<Part> {
    let mut parts = vec![];
    parse_parts_line_rec(line.as_bytes(), row, 0, &mut parts, None);
    parts
}

fn get_parts(lines: Vec<String>, gear_set: &mut HashMap<(usize, usize), Vec<u64>>) -> u64 {
    lines
        .iter()
        .map(Deref::deref)
        .enumerate()
        .flat_map(parse_parts_line)
        .filter_map(|p| p.validate(&lines, gear_set).then_some(p))
        .map(|p| p.id())
        .sum()
}

fn main() {
    let lines = aoc23::read_lines("../inputs/day03.txt");
    let mut gear_set = HashMap::new();
    let sum = get_parts(lines, &mut gear_set);
    println!("total part1: {sum}");

    let sum: u64 = gear_set
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v.iter().product::<u64>())
        .sum();
    println!("total part2: {sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let lines = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .map(String::from)
        .to_vec();

        let sum = get_parts(lines, &mut HashMap::new());

        assert_eq!(sum, 4361);
    }

    #[test]
    fn parse_line() {
        let line = "./467.*...@114";
        let parts = parse_parts_line((1, line));
        assert_eq!(
            parts,
            vec![
                Part {
                    start: 2,
                    end: 5,
                    row: 1,
                    val: b"467".to_vec(),
                },
                Part {
                    start: 11,
                    end: 14,
                    row: 1,
                    val: b"114".to_vec(),
                },
            ]
        )
    }
}
