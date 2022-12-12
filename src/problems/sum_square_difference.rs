// https://projecteuler.net/problem=6
pub const PROBLEM_ID: u16 = 6;

// TODO: The following can be done way better.

pub fn run() -> u64 {
    let range = 1u64..=100u64;

    let mut sum_square: u64 = 0;

    range.clone().all( |i| {
        sum_square += i.pow(2);
        true
    });

    let range_sum: u64 = range.sum();

    let square_sum: u64 = range_sum.pow(2);

    square_sum - sum_square
}
