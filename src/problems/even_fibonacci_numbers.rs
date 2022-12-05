// https://projecteuler.net/problem=2
pub const PROBLEM_ID: u16 = 2;

pub fn run() -> u64 {
    let mut result: u64 = 0;
    let mut buffer: [u64; 2] = [1, 0]; // this will hold the current number at [0] and the last one at [1]

    while buffer[0] < 4000000 {
        // do while the current number in the buffer is under 4 million
        buffer = [buffer.iter().sum(), buffer[0]]; // move the current number to last [1] and put the sum of both as the current number

        if buffer[0] % 2 == 0 {
            // when the current number is even
            result += buffer[0]; // add it to the result
        }
    }

    result
}
