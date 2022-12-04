pub const PROBLEM_ID: u16 = 3;

fn recur(n: u64, prime_factors: &mut Vec<u64>) {
    for i in 2..n {
        if n % i == 0 {
            prime_factors.push(i);
            recur(n / i, prime_factors);
            return
        }
    }

    prime_factors.push(n);
}

pub fn run() -> u64 {
    let mut prime_factors: Vec<u64> = Vec::new();

    recur(600851475143, &mut prime_factors);

    *prime_factors.iter().max().unwrap()
}