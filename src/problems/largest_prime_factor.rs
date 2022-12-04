// https://projecteuler.net/problem=3
pub const PROBLEM_ID: u16 = 3;

fn get_prime_factors(number: u64, prime_factors: &mut Vec<u64>) {
    for i in 2..number { // for all numbers between 2 and number
        if number % i == 0 { // when the current i is divisible by number
            prime_factors.push(i); // i is a prime factor - push to list of prime factors
            get_prime_factors(number / i, prime_factors); // recur
            return
        }
    }

    prime_factors.push(number); // this will catch the final prime factor
}

pub fn run() -> u64 {
    let mut prime_factors: Vec<u64> = Vec::new(); // all prime factors will be stored in this

    get_prime_factors(600851475143, &mut prime_factors);

    *prime_factors.iter().max().unwrap() // iterates over all the results, returns the largest number unwrapped
}