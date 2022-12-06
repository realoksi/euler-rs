pub mod even_fibonacci_numbers;
pub mod largest_palindrome_product;
pub mod largest_prime_factor;
pub mod multiples_of_3_or_5;
pub mod smallest_multiple;

use std::collections::HashMap;

pub fn run() {
    println!("...");
    let mut map: HashMap<u16, u64> = HashMap::new();

    map.insert(multiples_of_3_or_5::PROBLEM_ID, multiples_of_3_or_5::run());
    map.insert(
        even_fibonacci_numbers::PROBLEM_ID,
        even_fibonacci_numbers::run(),
    );
    map.insert(
        largest_prime_factor::PROBLEM_ID,
        largest_prime_factor::run(),
    );
    map.insert(
        largest_palindrome_product::PROBLEM_ID,
        largest_palindrome_product::run(),
    );
    map.insert(smallest_multiple::PROBLEM_ID, smallest_multiple::run());

    for key in 1u16..u16::MAX {
        match map.get(&key) {
            Some(v) => println!("{}. {}", key, v),
            None => break,
        };
    }
}
