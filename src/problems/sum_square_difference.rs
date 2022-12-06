// https://projecteuler.net/problem=6
pub const PROBLEM_ID: u16 = 6;

pub fn run() -> (u32, u32) {
    let range = 1u32..=100u32;
    let range_sum: u32 = range.sum();
    let range_pow: u32 = range_sum.pow(2);

    (range_pow - range_sum, range_pow)
}
