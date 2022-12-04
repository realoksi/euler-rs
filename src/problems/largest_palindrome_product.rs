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

pub fn run() -> u32 {
    let mut results: Vec<u32> = Vec::new();

    for i in (100..999).rev() {
        for x in (100..999).rev() {
            let s = (i*x).to_string();

            if s.len() % 2 == 0 { // if even
                let reversed : String = (&s[(s.len()/2)..]).chars().rev().collect();
                if &s[..(s.len()/2)] == reversed {
                    results.push(i*x);
                }

            } else {
                let reversed : String = (&s[(s.len()/2)+1..]).chars().rev().collect();

                if &s[..(s.len()/2)] == reversed {
                    results.push(i*x);

                }
            }
        }
    }

    *results.iter().max().unwrap()
}