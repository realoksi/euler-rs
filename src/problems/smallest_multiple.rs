// https://projecteuler.net/problem=5
pub const PROBLEM_ID: u16 = 5;

pub fn run() -> i32 {
    let mut multiple: i32 = 20;

    loop {
        if (1..=20).all(|x| -> bool {
            return multiple % x == 0
        }) {
            break;
        }

        multiple += 20;
    }

    multiple
}
