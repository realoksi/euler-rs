// https://projecteuler.net/problem=6
pub const PROBLEM_ID: u16 = 6;

pub fn run() -> u64 {
    const RANGE: std::ops::RangeInclusive<u64> = 1..=100;

    let mut sum_square: u64 = 0;
    for i in RANGE.clone() {
        sum_square += i.pow(2);
    }

    RANGE.sum::<u64>().pow(2) - sum_square
}
