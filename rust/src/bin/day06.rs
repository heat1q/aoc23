fn find_records((time, distance): (u64, u64)) -> u64 {
    (1..time)
        .map(|d| (time - d) * d)
        .filter(|d| d > &distance)
        .count() as u64
}

fn count_races(lines: &[String]) -> Option<u64> {
    let mut lines = lines.iter();
    let times = lines
        .next()?
        .split_once(':')?
        .1
        .split(' ')
        .filter_map(|v| v.parse::<u64>().ok());

    let distances = lines
        .next()?
        .split_once(':')?
        .1
        .split(' ')
        .filter_map(|v| v.parse::<u64>().ok());

    times
        .zip(distances)
        .map(find_records)
        .product::<u64>()
        .into()
}

fn main() {
    let lines = aoc23::read_lines("../inputs/day06.txt");
    let count = count_races(&lines).unwrap();
    println!("part_1/part_2 {count}");
}
