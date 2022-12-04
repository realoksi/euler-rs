pub mod multiples_of_3_or_5;
pub mod even_fibonacci_numbers;
pub mod largest_prime_factor;

// TODO: Implement a more automated method for running each problem.

pub fn run() {
    println!("{}. {}", multiples_of_3_or_5::PROBLEM_ID, multiples_of_3_or_5::run());
    println!("{}. {}", even_fibonacci_numbers::PROBLEM_ID, even_fibonacci_numbers::run());
    println!("{}. {}", largest_prime_factor::PROBLEM_ID, largest_prime_factor::run());
}