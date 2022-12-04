// https://projecteuler.net/problem=4
pub const PROBLEM_ID: u16 = 4;

fn get_digits(number: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();

    fn push_digits(number: u32, digits: &mut Vec<u32>) {
        if number >= 10 {
            push_digits(number / 10, digits);
        }

        digits.push(number % 10);
    }

    push_digits(number, &mut digits);

    digits
}

fn is_palindrome ( number: u32 ) -> bool {
    let digits = get_digits(number);

    for i in 0..=digits.len()/2 {
        if digits[i] != digits[(digits.len() - 1)-i] {
            return false
        }
    }

    true
}

pub fn run() -> u32 {
    let mut results: Vec<u32> = Vec::new();

    for i in (100u32..999u32).rev() {
        for x in (100u32..999u32).rev() {
            let number: u32 = i*x;

            if !is_palindrome(number) {
                continue;
            }

            results.push(number);
        }
    }

    *results.iter().max().unwrap()
}