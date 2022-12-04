// https://projecteuler.net/problem=1
pub const PROBLEM_ID: u16 = 1;

pub fn run() -> u32 {
    let mut result: u32 = 0;

    for i in 1..1000 {
        // for all numbers between 0 and 1000
        if [3, 5].iter().any(|x| i % x == 0) {
            // when either 3 or 5 are divisible by i
            result += i; // add i to the result
        }
    }

    result
}
