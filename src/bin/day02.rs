#[derive(Debug)]
struct Range {
    low: u64,
    high: u64,
}

fn parse_input(input: &str) -> Vec<Range> {
    input
        .split(',')
        .map(|r| {
            let (low, high) = r.split_once('-').expect("invalid range");
            Range {
                low: low.trim().parse().expect("invalid low number"),
                high: high.trim().parse().expect("invalid high number"),
            }
        })
        .collect()
}

fn num_digits(n: u64) -> u64 {
    if n == 0 { 1 } else { n.ilog10() as u64 + 1 }
}

fn split_num_half(num: &str) -> (u64, u64) {
    let n = num.len();
    assert!(n % 2 == 0, "str {} length must be even", num);
    (
        num[..(n / 2)].parse().expect("invalid number"),
        num[(n / 2)..].parse().expect("invalid number"),
    )
}

/// Computes the sum 1 + 2 + ... + n
fn triangle_number(n: u64) -> u64 {
    (n * n + n) / 2
}

/// Computes low + low + 1 + ... + high
fn sum_low_high(low: u64, high: u64) -> u64 {
    assert!(
        low <= high,
        "low must be <= high. got low:{} high:{}",
        low,
        high
    );
    assert!(low > 0, "low must be > 0. got low:{}", low);

    triangle_number(high) - triangle_number(low - 1)
}

/// Given low and high with same number of digits, computes the sum
/// (low)(low) + (low+1)(low+1) + ... + (high)(high)
///
/// (x)(x) is the number formed by concatening the digits of x.
/// E.g. x = 1232, (x)(x) = 12321232
///
/// If the number of digits of low and high is n
/// the required sum is (10^n + 1) * (low + low + 1 + ... + high)
///
/// This is a special case of num_x_sum where x = 2
fn numnum_sum(low: u64, high: u64) -> u64 {
    num_x_sum(low, high, 2)
}

/// Let rep(n, x) = the number n concatenated x times.
/// E.g. rep(23, 3) = 232323
///
/// This function computes the sum rep(low, x) + rep(low+1, x) + ... + rep(high, x)
/// Assertions: num_digits(low) == num_digits(high) = d
/// Required sum = (1 + 10^d + 10^2d + ... + 10^(x-1)d) * (low + low + 1 + ... + high)
fn num_x_sum(low: u64, high: u64, x: u64) -> u64 {
    assert!(
        low <= high,
        "low must be <= high. got low:{} high:{}",
        low,
        high
    );
    assert_eq!(
        num_digits(low),
        num_digits(high),
        "number of digits in low and high must be same"
    );
    let d = num_digits(low);

    // Compute (1 + 10^d + ... + 10^(x-1)d) = (10^xd - 1)/(10^d - 1)
    let pow10_sum: u64 = (10u64.pow((x * d) as u32) - 1) / (10u64.pow(d as u32) - 1);
    pow10_sum * sum_low_high(low, high)
}

struct SmallDivisors {
    n: u64,
    current: u64,
}

impl SmallDivisors {
    fn new(n: u64) -> Self {
        Self { n, current: 1 }
    }
}

impl Iterator for SmallDivisors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current * self.current <= self.n {
            let poss_divisor = self.current;
            self.current += 1;

            if self.n % poss_divisor == 0 {
                return Some(poss_divisor);
            }
        }
        None
    }
}

fn part1(input: &str) -> u64 {
    let ranges: Vec<Range> = parse_input(input);
    let mut ans: u64 = 0;

    for range in ranges {
        let low_str = range.low.to_string();
        let high_str = range.high.to_string();
        let low_str_len = low_str.len() as u32;
        let high_str_len = high_str.len() as u32;
        let start = if low_str_len % 2 == 0 {
            low_str_len
        } else {
            low_str_len + 1
        };

        for n in (start..=high_str_len).step_by(2) {
            let num_digits = n / 2;
            let mut low_half: u64 = 10u64.pow(num_digits - 1);
            let mut high_half: u64 = 10u64.pow(num_digits) - 1;

            if n == low_str_len {
                let (first_half, second_half) = split_num_half(&low_str);
                low_half = if second_half <= first_half {
                    first_half
                } else {
                    first_half + 1
                }
            }
            if n == high_str_len {
                let (first_half, second_half) = split_num_half(&high_str);
                high_half = if second_half >= first_half {
                    first_half
                } else {
                    first_half - 1
                }
            }

            if low_half <= high_half {
                ans += numnum_sum(low_half, high_half);
            }
        }
    }

    ans
}

fn is_invalid(n: u64) -> bool {
    let d = num_digits(n);
    SmallDivisors::new(d)
        .any(|x| is_x_repeating(n, x as u32) || is_x_repeating(n, (d / x) as u32))
}

/// Given n with d digits and x, x | d checks if n is formed by a repeating sequence of x digits
/// NOTE: if x == d, false is returned!!!
fn is_x_repeating(n: u64, x: u32) -> bool {
    let d: u32 = num_digits(n) as u32;
    assert!(
        d % x == 0,
        "x must divide the number of digits of n. x:{x}, n:{n}, d:{d}"
    );

    if x == d {
        return false;
    }

    let first_pow = 10u64.pow(x);
    let first_chunk = n % first_pow;
    let num_chunks = d / x;
    (1..num_chunks).all(|i| {
        let pow = 10u64.pow(i * x);
        let chunk = (n / pow) % first_pow;
        chunk == first_chunk
    })
}

fn part2(input: &str) -> u64 {
    let ranges: Vec<Range> = parse_input(input);
    let ans: u64 = ranges
        .iter()
        .flat_map(|r| r.low..=r.high)
        .filter(|&n| is_invalid(n))
        .sum();

    ans
}

fn main() {
    const INPUT: &str = include_str!("../../inputs/day02.txt");
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../inputs/day02_example.txt");

    #[test]
    fn test1() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test2() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn test_is_x_repeating() {
        assert_eq!(is_x_repeating(111, 1), true);
        assert_eq!(is_x_repeating(123, 1), false);
        assert_eq!(is_x_repeating(111, 3), false); // !expected!
        assert_eq!(is_x_repeating(12341234, 2), false);
        assert_eq!(is_x_repeating(12341234, 4), true);
        assert_eq!(is_x_repeating(12121212, 2), true);
        assert_eq!(is_x_repeating(12121212, 4), true);
    }

    #[test]
    fn test_is_invalid() {
        assert_eq!(is_invalid(123123), true);
        assert_eq!(is_invalid(123456), false);
        assert_eq!(is_invalid(111), true);
        assert_eq!(is_invalid(12121212), true);
        assert_eq!(is_invalid(12121221), false);
    }
}
