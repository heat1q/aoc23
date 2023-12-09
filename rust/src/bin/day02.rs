use std::ops::Deref;

use aoc23 as aoc_lib;

#[derive(Debug, Default, PartialEq)]
struct Game {
    blue: u32,
    green: u32,
    red: u32,
}

impl Game {
    fn from_str(input: &str) -> Self {
        let mut g = Self::default();
        input
            .split([',', ' '])
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .chunks(2)
            .filter_map(|c| Some((c[0].parse::<u32>().ok()?, c[1])))
            .for_each(|(count, color)| {
                match color {
                    "blue" => g.blue = count,
                    "green" => g.green = count,
                    "red" => g.red = count,
                    _ => (),
                };
            });
        g
    }

    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn power(&self) -> u32 {
        if self.red + self.green + self.blue == 0 {
            return 0;
        }
        let mut p = 1;
        if self.red != 0 {
            p *= self.red;
        }
        if self.green != 0 {
            p *= self.green;
        }
        if self.red != 0 {
            p *= self.blue;
        }
        p
    }

    fn max(mut self, rhs: Game) -> Self {
        self.blue = std::cmp::max(self.blue, rhs.blue);
        self.green = std::cmp::max(self.green, rhs.green);
        self.red = std::cmp::max(self.red, rhs.red);
        self
    }
}

fn validate_game(line: &str) -> Option<u32> {
    let (id, game) = line.split_once(':')?;
    game.split(';')
        .map(Game::from_str)
        .map(|g| g.is_valid())
        .all(bool::from)
        .then_some(id)
        .and_then(|id| {
            let (_, id) = id.split_once(' ')?;
            id.parse().ok()
        })
}

fn minimum_power_set(line: &str) -> Option<u32> {
    let (_, game) = line.split_once(':')?;
    game.split(';')
        .map(Game::from_str)
        .fold(Game::default(), |acc, n| acc.max(n))
        .power()
        .into()
}

fn main() {
    let lines = aoc_lib::read_lines("../inputs/day02.txt");
    let part_1: u32 = lines
        .iter()
        .map(Deref::deref)
        .filter_map(validate_game)
        .sum();
    println!("part_1 {part_1} (2617)");

    let part_2: u32 = lines
        .iter()
        .map(Deref::deref)
        .filter_map(minimum_power_set)
        .sum();
    println!("part_2 {part_2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_single_game() {
        //let input = "1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let input = "3 green, 4 blue, 1 red";
        let s = input.split([',', ' ']).collect::<Vec<&str>>();
        println!("{s:?}");
        let g = Game::from_str(input);
        assert_eq!(
            g,
            Game {
                red: 1,
                green: 3,
                blue: 4
            }
        );
    }

    #[test]
    fn minimum_power_set_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let p = minimum_power_set(input);
        assert_eq!(p, Some(48))
    }
}
