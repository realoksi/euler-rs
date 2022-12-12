pub mod even_fibonacci_numbers;
pub mod largest_palindrome_product;
pub mod largest_prime_factor;
pub mod multiples_of_3_or_5;
pub mod smallest_multiple;
pub mod sum_square_difference;
// ...
pub mod largest_product_in_a_series;

pub fn run() {
    println!(
        "{}. {}",
        multiples_of_3_or_5::PROBLEM_ID,
        multiples_of_3_or_5::run()
    );
    println!(
        "{}. {}",
        even_fibonacci_numbers::PROBLEM_ID,
        even_fibonacci_numbers::run()
    );
    println!(
        "{}. {}",
        largest_prime_factor::PROBLEM_ID,
        largest_prime_factor::run()
    );
    println!(
        "{}. {}",
        largest_palindrome_product::PROBLEM_ID,
        largest_palindrome_product::run()
    );
    println!(
        "{}. {}",
        smallest_multiple::PROBLEM_ID,
        smallest_multiple::run()
    );
    println!(
        "{}. {:?}",
        sum_square_difference::PROBLEM_ID,
        sum_square_difference::run()
    );
    println!(
        "{}. {}",
        largest_product_in_a_series::PROBLEM_ID,
        largest_product_in_a_series::run()
    );
}
