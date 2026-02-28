use std::collections::VecDeque;

/// (x, y) deltas to get the neighbouring cells in a 2D grid
const NEIGHBOURS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

type Grid = Vec<Vec<char>>;

fn parse_input(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn calc_accessible(grid: &Grid) -> u32 {
    grid.iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .filter(|&(col_index, &ch)| {
                    ch == '@' && count_neighbours(grid, row_index, col_index) < 4
                })
                .count() as u32
        })
        .sum::<u32>()
}

fn count_neighbours(grid: &Grid, row_index: usize, col_index: usize) -> u32 {
    let num_rows = grid.len();
    assert!(num_rows > 0, "grid must be non-empty");
    let num_cols = grid[0].len();
    assert!(num_cols > 0, "grid must be non-empty");

    NEIGHBOURS
        .iter()
        .map(|(dx, dy)| (row_index as isize + dx, col_index as isize + dy))
        .filter(|&(r, c)| {
            in_bounds(r, c, num_rows, num_cols) && grid[r as usize][c as usize] == '@'
        })
        .count() as u32
}

/// Simulates removal of paper rolls and returns the number of rolls removed
fn simulate_removal(grid: &Grid) -> u32 {
    let mut num_removed = 0;
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut cell_queue: VecDeque<(usize, usize)> = VecDeque::new();

    // Fill the queue for processing paper rolls
    for r in 0..num_rows {
        for c in 0..num_cols {
            if grid[r][c] == '@' {
                cell_queue.push_back((r, c));
            }
        }
    }

    let mut grid_working = grid.clone();
    while let Some((r, c)) = cell_queue.pop_front() {
        // If this cell has already been processed, or if it has >= 4 neighbours, ignore
        if grid_working[r][c] != '@' || count_neighbours(&grid_working, r, c) >= 4 {
            continue;
        }

        // Remove this cell, as it has less than 4 neighbours
        grid_working[r][c] = '.';
        num_removed += 1;

        // Its neighbours can be potentially removed, check and add them to the queue for
        // processing
        for (dx, dy) in NEIGHBOURS {
            let x = r as isize + dx;
            let y = c as isize + dy;

            if in_bounds(x, y, num_rows, num_cols) {
                let x = x as usize;
                let y = y as usize;
                if grid_working[x][y] == '@' && count_neighbours(&grid_working, x, y) < 4 {
                    cell_queue.push_back((x, y));
                }
            }
        }
    }

    num_removed
}

fn in_bounds(x: isize, y: isize, num_rows: usize, num_cols: usize) -> bool {
    x >= 0 && x < num_rows as isize && y >= 0 && y < num_cols as isize
}

fn part1(input: &str) -> u32 {
    let grid = parse_input(input);
    calc_accessible(&grid)
}

fn part2(input: &str) -> u32 {
    let grid = parse_input(input);
    simulate_removal(&grid)
}

fn main() {
    const INPUT: &str = include_str!("../../inputs/day04.txt");
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../inputs/day04_example.txt");

    #[test]
    fn test1() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 13);
    }

    #[test]
    fn test2() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 43);
    }
}
