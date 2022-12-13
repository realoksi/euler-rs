// https://projecteuler.net/problem=5
pub const PROBLEM_ID: u16 = 5;

pub fn run() -> u32 {
    const RANGE: std::ops::RangeInclusive<u32> = 1..=20;
    let mut multiple = *RANGE.end();

    while multiple <= u32::MAX {
        if RANGE.into_iter().all(|f| multiple % f == 0) {
            break;
        }

        multiple += 20;
    }

    return multiple;
}
