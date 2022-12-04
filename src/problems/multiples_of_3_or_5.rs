pub const PROBLEM_ID: u16 = 1;

pub fn run() -> i32 {
    let mut result: i32 = 0;

    for i in 0..1000 {
        if [3, 5].iter().any(|x| i % x == 0) {
            result += i;
        }
    }

    result
}