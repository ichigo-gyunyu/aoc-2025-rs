const START_POS: i32 = 50;
const DIAL_SIZE: i32 = 100;

fn parse_line(line: &str) -> (i32, i32) {
    let (direction, distance) = line.split_at(1);
    let distance: i32 = distance.parse().unwrap();
    match direction {
        "L" => (-1, distance),
        "R" => (1, distance),
        _ => panic!("unknown direction"),
    }
}

fn part1(input: &str) -> i32 {
    let mut pos = START_POS;
    let mut password = 0;

    for line in input.lines() {
        let (direction, distance) = parse_line(line);
        pos = match direction {
            -1 => (pos - distance + DIAL_SIZE) % DIAL_SIZE,
            1 => (pos + distance) % DIAL_SIZE,
            _ => panic!("unknown direction)"),
        };
        if pos == 0 {
            password += 1;
        }
    }
    password
}

fn part2(input: &str) -> i32 {
    let mut pos = START_POS;
    let mut password = 0;

    for line in input.lines() {
        let (direction, distance) = parse_line(line);

        let num_full_rotations = distance / DIAL_SIZE;
        password += num_full_rotations;

        let extra_distance = distance % DIAL_SIZE;
        let prev_pos = pos;
        pos += direction * extra_distance;
        if prev_pos != 0 && extra_distance > 0 && (pos <= 0 || pos >= DIAL_SIZE) {
            password += 1;
        }
        pos = (pos + DIAL_SIZE) % DIAL_SIZE;
    }

    password
}

fn main() {
    const INPUT: &str = include_str!("../../inputs/day01.txt");
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../inputs/day01_example.txt");

    #[test]
    fn test1() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 3);
    }

    #[test]
    fn test2() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 6);
    }
}
