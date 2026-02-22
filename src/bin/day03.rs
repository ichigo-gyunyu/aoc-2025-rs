type Bank = Vec<u8>;

fn parse_input(input: &str) -> Vec<Bank> {
    input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect()
}

fn max_joltage(bank: &Bank) -> u32 {
    let (first_index, first_value) = max_val_and_pos(&bank[..bank.len() - 1]);
    let (_, second_value) = max_val_and_pos(&bank[first_index + 1..]);

    (first_value as u32) * 10 + (second_value as u32)
}

/// Returns the index and max value of a slice.
/// If several elements are equally maximum, the first index is returned
fn max_val_and_pos(arr: &[u8]) -> (usize, u8) {
    let (index, &value) = arr
        .iter()
        .rev()
        .enumerate()
        .max_by_key(|(_, v)| *v)
        .expect("array must not be empty");

    (arr.len() - 1 - index, value)
}

fn max_joltage_12(bank: &Bank) -> u64 {
    const NUM_BATTERIES: usize = 12;
    let mut left_index: usize = 0;

    (1..=NUM_BATTERIES).rev().fold(0, |acc, battery_num| {
        let right_index = bank.len() - (battery_num - 1);
        let (index, value) = max_val_and_pos(&bank[left_index..right_index]);
        left_index += index + 1;
        acc * 10 + (value as u64)
    })
}

fn part1(input: &str) -> u32 {
    let banks: Vec<Bank> = parse_input(input);
    banks.iter().map(|bank| max_joltage(bank)).sum()
}

fn part2(input: &str) -> u64 {
    let banks: Vec<Bank> = parse_input(input);
    banks.iter().map(|bank| max_joltage_12(bank)).sum()
}

fn main() {
    const INPUT: &str = include_str!("../../inputs/day03.txt");
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../inputs/day03_example.txt");

    #[test]
    fn test1() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 357);
    }

    #[test]
    fn test2() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 3121910778619);
    }
}
