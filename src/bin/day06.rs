use std::iter::zip;

type Operation = char;

fn parse_input1(input: &str) -> (Vec<Vec<u64>>, Vec<Operation>) {
    let mut it = input.lines().rev();

    let operations: Vec<Operation> = it
        .next()
        .expect("invalid input")
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    let mut operands_list: Vec<Vec<u64>> = vec![Vec::new(); operations.len()];
    for line in it {
        for (i, operand) in line.split_whitespace().enumerate() {
            operands_list[i].push(operand.parse().expect("invalid input"));
        }
    }

    (operands_list, operations)
}

fn parse_input2(input: &str) -> (Vec<Vec<u64>>, Vec<Operation>) {
    let operations = input
        .lines()
        .last()
        .expect("invalid input")
        .chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .collect::<Vec<Operation>>();

    let operands_grid = input
        .strip_suffix('\n')
        .unwrap_or(input)
        .rsplit_once('\n')
        .expect("invalid input")
        .0
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let num_cols = operands_grid[0].len();
    assert!(operands_grid.iter().all(|row| row.len() == num_cols));

    // rotate
    let operands_grid = (0..num_cols)
        .rev()
        .map(|c| {
            operands_grid
                .iter()
                .map(|row| row[c])
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    let operands_list = operands_grid
        .split(|op| op.is_empty())
        .map(|ops| {
            ops.iter()
                .map(|op| op.parse().expect("invalid input"))
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    (operands_list, operations)
}

fn solve_arithmetic((operands_list, operations): (Vec<Vec<u64>>, Vec<Operation>)) -> u64 {
    zip(operands_list, operations)
        .map(|(operands, operation)| -> u64 {
            match operation {
                '+' => operands.iter().sum(),
                '*' => operands.iter().product(),
                _ => panic!("invalid operation"),
            }
        })
        .sum()
}

fn part1(input: &str) -> u64 {
    solve_arithmetic(parse_input1(input))
}

fn part2(input: &str) -> u64 {
    solve_arithmetic(parse_input2(input))
}

fn main() {
    const INPUT: &str = include_str!("../../inputs/day06.txt");
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../inputs/day06_example.txt");

    #[test]
    fn test1() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test2() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 3263827);
    }
}
