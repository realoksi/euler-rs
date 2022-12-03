fn multiples_of_3_or_5() {
    let mut r: i32 = 0;

    for i in 0..1000 {
        if [3, 5].iter().any(|x| i % x == 0) {
            r += i;
        }
    }

    println!("1. {}", r);
}

fn even_fibonacci_numbers() {
    let mut r: u64 = 0;
    let mut last: [u64; 2] = [0, 1];

    while last[1] < 4000000 {
        last = [last[1], last.iter().sum()];

        if last[1] % 2 == 0 {
            r += last[1];
        }
    }

    println!("2. {}", r);
}

fn prime_factors_recurrsive(n: u64, prime_factors: &mut Vec<u64>) {
    for i in 2..n {
        if n % i == 0 {
            prime_factors.push(i);
            prime_factors_recurrsive(n / i, prime_factors);
            return
        }
    }

    prime_factors.push(n);
}

fn largest_prime_factor() {
    let mut prime_factors: Vec<u64> = Vec::new();

    prime_factors_recurrsive(600851475143, &mut prime_factors);

    println!("3. {}", *prime_factors.iter().max().unwrap());
}

fn main() {
    multiples_of_3_or_5();
    even_fibonacci_numbers();
    largest_prime_factor();
}
