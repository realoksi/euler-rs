pub const PROBLEM_ID: u16 = 2;

pub fn run() -> u64 {
    let mut result: u64 = 0;
    let mut last: [u64; 2] = [0, 1];

    while last[1] < 4000000 {
        last = [last[1], last.iter().sum()];

        if last[1] % 2 == 0 {
            result += last[1];
        }
    }

    result
}