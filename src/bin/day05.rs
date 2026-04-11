use aoc2025::utils::interval_set::{Interval, IntervalSet};

type IngredientId = u64;
type IngredientIdRange = Interval;

fn parse_input(input: &str) -> (Vec<IngredientIdRange>, Vec<IngredientId>) {
    let (ranges, ids) = input
        .split_once("\n\n")
        .expect("invalid input, no blank line found.");

    let ranges = ranges
        .lines()
        .map(|range| {
            let (l, r) = range.split_once('-').expect("invalid range format");
            Interval::new(
                l.parse().expect("invalid left bound"),
                r.parse().expect("invalid right bound"),
            )
        })
        .collect();

    let ids = ids
        .lines()
        .map(|id| id.parse().expect("invalid ingredient id"))
        .collect();

    (ranges, ids)
}

fn part1(input: &str) -> u64 {
    let (ranges, ids) = parse_input(input);

    // construct an interval set from the ingredient ranges
    let is = IntervalSet::from(ranges);

    // iterate over the ingredient ids to count the ones that are fresh (contained in the interval
    // set)
    ids.iter().filter(|&&id| is.is_contained(id)).count() as u64
}

fn part2(input: &str) -> u64 {
    let (ranges, _) = parse_input(input);

    // construct an interval set from the ingredient ranges
    let is = IntervalSet::from(ranges);

    // return the number total number of "fresh" ingredients, which is the total length of the
    // interval set
    is.total_length()
}

fn main() {
    const INPUT: &str = include_str!("../../inputs/day05.txt");
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../inputs/day05_example.txt");

    #[test]
    fn test1() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 14);
    }
}
