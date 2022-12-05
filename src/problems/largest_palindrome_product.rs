// https://projecteuler.net/problem=4
pub const PROBLEM_ID: u16 = 4;

fn get_digits(number: u64) -> Vec<u64> {
    let mut digits: Vec<u64> = Vec::new();

    fn push_digits(number: u64, digits: &mut Vec<u64>) {
        if number >= 10 {
            push_digits(number / 10, digits);
        }

        digits.push(number % 10);
    }

    push_digits(number, &mut digits);

    digits
}

fn is_palindrome(number: u64) -> bool {
    let digits = get_digits(number);

    for i in 0..=digits.len() / 2 {
        if digits[i] != digits[(digits.len() - 1) - i] {
            return false;
        }
    }

    true
}

pub fn run() -> u64 {
    let mut results: Vec<u64> = Vec::new();

    for i in (100u64..999u64).rev() {
        for x in (100u64..999u64).rev() {
            let number: u64 = i * x;

            if is_palindrome(number) {
                results.push(number);
            }
        }
    }

    *results.iter().max().unwrap()
}
