// https://projecteuler.net/problem=7
pub const PROBLEM_ID: u16 = 7;
const MAX : usize = 10001;

pub fn run() -> u32 {
    let mut marked: Vec<u32> = Vec::with_capacity(MAX);

    for i in 2..u32::MAX {
        if marked.iter().any(|f| i % f == 0 ){
            continue;
        }

        marked.push(i);

        if marked.len() >= MAX {
            break;
        }
    }

    *marked.last().unwrap()
}
