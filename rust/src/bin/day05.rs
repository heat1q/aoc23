use std::ops::Deref;

#[derive(Debug)]
struct RangeMapper {
    start: i64,
    end: i64,
    delta: i64,
}

impl RangeMapper {
    fn from_input(line: &str) -> Self {
        let [dest, src, range] = line
            .split(' ')
            .filter_map(|s| s.parse::<i64>().ok())
            .collect::<Vec<i64>>()[..]
        else {
            unreachable!()
        };

        Self {
            start: src,
            end: src + range,
            delta: dest - src,
        }
    }

    fn map_if(&self, src: i64) -> Option<i64> {
        if src >= self.start && src < self.end {
            Some(src + self.delta)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<RangeMapper>,
}

impl Map {
    fn from_input(lines: &[String]) -> Self {
        let ranges = lines
            .iter()
            .skip(1)
            .map(Deref::deref)
            .map(RangeMapper::from_input)
            .collect();
        Self { ranges }
    }

    fn map(&self, src: i64) -> i64 {
        self.ranges.iter().fold(src, |acc, n| match n.map_if(src) {
            Some(dest) => dest,
            None => acc,
        })
    }
}

fn find_lowest_location(lines: &[String]) -> Option<i64> {
    let mut lines_iter = lines.split(String::is_empty);
    let seeds: Vec<i64> = lines_iter
        .next()
        .map_or("", |s| s[0].as_str())
        .split(' ')
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();
    let mapper = lines_iter.map(Map::from_input).collect::<Vec<Map>>();
    println!("mapper: {mapper:?}");

    seeds
        .iter()
        .copied()
        .map(|s| mapper.iter().fold(s, |acc, n| n.map(acc)))
        .min()
}

fn main() {
    let lines = aoc23::read_lines("../inputs/day05.txt");
    let part_1 = find_lowest_location(&lines).unwrap();
    println!("part_1 {part_1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map() {
        let input = ["seed-to-soil map:", "50 98 2", "52 50 48"].map(String::from);
        let mapper = Map::from_input(&input);

        assert_eq!(mapper.map(98), 50);
    }

    #[test]
    fn range_mapper() {
        let input = "50 98 2";
        let range_mapper = RangeMapper::from_input(input);

        assert_eq!(range_mapper.map_if(97), None);
        assert_eq!(range_mapper.map_if(98), Some(50));
        assert_eq!(range_mapper.map_if(99), Some(51));
        assert_eq!(range_mapper.map_if(100), None);
    }
}
