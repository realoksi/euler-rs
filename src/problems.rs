use std::fmt;

trait Problem<T: fmt::Display + std::cmp::PartialEq + 'static> {
    fn title(&self) -> &'static str;
    fn id(&self) -> &'static u16;
    fn answer(&self) -> &'static T;
    fn run(&self) -> T;
    fn test(&self) -> bool {
        let result = self.run() == *self.answer();

        if result {
            println!(
                "\x1b[38;2;62;247;105m{}. {}\x1b[0m 👍",
                self.id(),
                self.title().to_lowercase()
            );
        } else {
            println!(
                "\x1b[38;2;255;18;73m{}. {}\x1b[0m 👎",
                self.id(),
                self.title().to_lowercase()
            );
        }

        result
    }
}

struct P1;
impl Problem<i32> for P1 {
    fn title(&self) -> &'static str {
        "Multiples of 3 or 5"
    }

    fn id(&self) -> &'static u16 {
        &1
    }

    fn answer(&self) -> &'static i32 {
        &0x38ED0
    }

    fn run(&self) -> i32 {
        let mut r = 0;

        for i in 1..1000 {
            if [3, 5].iter().any(|x| i % x == 0) {
                r += i;
            }
        }

        r
    }
}

struct P2;
impl Problem<u32> for P2 {
    fn title(&self) -> &'static str {
        "Even Fibonacci Numbers"
    }

    fn id(&self) -> &'static u16 {
        &2
    }

    fn answer(&self) -> &'static u32 {
        &0x466664
    }

    fn run(&self) -> u32 {
        let mut r: u32 = 0;
        let mut b: [u32; 2] = [1, 0];

        while b[0] < 4000000 {
            b = [b.iter().sum(), b[0]];

            if b[0] % 2 == 0 {
                r += b[0];
            }
        }

        r
    }
}

struct P3;
impl Problem<u64> for P3 {
    fn title(&self) -> &'static str {
        "Largest Prime Factor"
    }

    fn id(&self) -> &'static u16 {
        &3
    }

    fn answer(&self) -> &'static u64 {
        &0x1AC9
    }

    fn run(&self) -> u64 {
        fn _r(n: u64, p: &mut Vec<u64>) {
            for i in 2..n {
                if n % i == 0 {
                    p.push(i);
                    _r(n / i, p);
                    return;
                }
            }

            p.push(n);
        }

        let mut p: Vec<u64> = Vec::new();

        _r(600851475143, &mut p);

        *p.iter().max().unwrap()
    }
}

struct P4;
impl Problem<u32> for P4 {
    fn title(&self) -> &'static str {
        "Largest Palindrome Product"
    }

    fn id(&self) -> &'static u16 {
        &4
    }

    fn answer(&self) -> &'static u32 {
        &0xDD571
    }

    fn run(&self) -> u32 {
        fn push_digits(n: u32, d: &mut Vec<u32>) {
            if n >= 10 {
                push_digits(n / 10, d);
            }

            d.push(n % 10);
        }

        fn get_digits(n: u32) -> Vec<u32> {
            let mut d: Vec<u32> = Vec::new();

            push_digits(n, &mut d);

            d
        }

        fn is_palindrome(n: u32) -> bool {
            let d = get_digits(n);

            for i in 0..=d.len() / 2 {
                if d[i] != d[(d.len() - 1) - i] {
                    return false;
                }
            }

            true
        }

        let mut r: Vec<u32> = Vec::new();

        for i in (100u32..999u32).rev() {
            for x in (100u32..999u32).rev() {
                let n: u32 = i * x;

                if is_palindrome(n) {
                    r.push(n);
                }
            }
        }

        *r.iter().max().unwrap()
    }
}

struct P5;
impl Problem<u32> for P5 {
    fn title(&self) -> &'static str {
        "Smallest Multiple"
    }

    fn id(&self) -> &'static u16 {
        &5
    }

    fn answer(&self) -> &'static u32 {
        &0xDE021F0
    }

    fn run(&self) -> u32 {
        let mut m = 20;

        while m <= u32::MAX {
            if (1..=20).into_iter().all(|f| m % f == 0) {
                break;
            }

            m += 20;
        }

        m
    }
}

struct P6;
impl Problem<u64> for P6 {
    fn title(&self) -> &'static str {
        "Sum Square Difference"
    }

    fn id(&self) -> &'static u16 {
        &6
    }

    fn answer(&self) -> &'static u64 {
        &0x17FF976
    }

    fn run(&self) -> u64 {
        let mut s: u64 = 0;
        for i in 1..=100u64 {
            s += i.pow(2);
        }

        (1..=100).sum::<u64>().pow(2) - s
    }
}

struct P7;
impl Problem<u32> for P7 {
    fn title(&self) -> &'static str {
        "10001st Prime"
    }

    fn id(&self) -> &'static u16 {
        &7
    }

    fn answer(&self) -> &'static u32 {
        &0x19927
    }

    fn run(&self) -> u32 {
        let mut m: Vec<u32> = Vec::with_capacity(10001);

        for i in 2..u32::MAX {
            if m.iter().any(|f| i % f == 0) {
                continue;
            }

            m.push(i);

            if m.len() >= 10001 {
                break;
            }
        }

        *m.last().unwrap()
    }
}

struct P8;
impl Problem<u64> for P8 {
    fn title(&self) -> &'static str {
        "Largest Product In A Series"
    }

    fn id(&self) -> &'static u16 {
        &8
    }

    fn answer(&self) -> &'static u64 {
        &0x57994B000
    }

    fn run(&self) -> u64 {
        const S: [u8; 1000] = [
            7, 3, 1, 6, 7, 1, 7, 6, 5, 3, 1, 3, 3, 0, 6, 2, 4, 9, 1, 9, 2, 2, 5, 1, 1, 9, 6, 7, 4,
            4, 2, 6, 5, 7, 4, 7, 4, 2, 3, 5, 5, 3, 4, 9, 1, 9, 4, 9, 3, 4, 9, 6, 9, 8, 3, 5, 2, 0,
            3, 1, 2, 7, 7, 4, 5, 0, 6, 3, 2, 6, 2, 3, 9, 5, 7, 8, 3, 1, 8, 0, 1, 6, 9, 8, 4, 8, 0,
            1, 8, 6, 9, 4, 7, 8, 8, 5, 1, 8, 4, 3, 8, 5, 8, 6, 1, 5, 6, 0, 7, 8, 9, 1, 1, 2, 9, 4,
            9, 4, 9, 5, 4, 5, 9, 5, 0, 1, 7, 3, 7, 9, 5, 8, 3, 3, 1, 9, 5, 2, 8, 5, 3, 2, 0, 8, 8,
            0, 5, 5, 1, 1, 1, 2, 5, 4, 0, 6, 9, 8, 7, 4, 7, 1, 5, 8, 5, 2, 3, 8, 6, 3, 0, 5, 0, 7,
            1, 5, 6, 9, 3, 2, 9, 0, 9, 6, 3, 2, 9, 5, 2, 2, 7, 4, 4, 3, 0, 4, 3, 5, 5, 7, 6, 6, 8,
            9, 6, 6, 4, 8, 9, 5, 0, 4, 4, 5, 2, 4, 4, 5, 2, 3, 1, 6, 1, 7, 3, 1, 8, 5, 6, 4, 0, 3,
            0, 9, 8, 7, 1, 1, 1, 2, 1, 7, 2, 2, 3, 8, 3, 1, 1, 3, 6, 2, 2, 2, 9, 8, 9, 3, 4, 2, 3,
            3, 8, 0, 3, 0, 8, 1, 3, 5, 3, 3, 6, 2, 7, 6, 6, 1, 4, 2, 8, 2, 8, 0, 6, 4, 4, 4, 4, 8,
            6, 6, 4, 5, 2, 3, 8, 7, 4, 9, 3, 0, 3, 5, 8, 9, 0, 7, 2, 9, 6, 2, 9, 0, 4, 9, 1, 5, 6,
            0, 4, 4, 0, 7, 7, 2, 3, 9, 0, 7, 1, 3, 8, 1, 0, 5, 1, 5, 8, 5, 9, 3, 0, 7, 9, 6, 0, 8,
            6, 6, 7, 0, 1, 7, 2, 4, 2, 7, 1, 2, 1, 8, 8, 3, 9, 9, 8, 7, 9, 7, 9, 0, 8, 7, 9, 2, 2,
            7, 4, 9, 2, 1, 9, 0, 1, 6, 9, 9, 7, 2, 0, 8, 8, 8, 0, 9, 3, 7, 7, 6, 6, 5, 7, 2, 7, 3,
            3, 3, 0, 0, 1, 0, 5, 3, 3, 6, 7, 8, 8, 1, 2, 2, 0, 2, 3, 5, 4, 2, 1, 8, 0, 9, 7, 5, 1,
            2, 5, 4, 5, 4, 0, 5, 9, 4, 7, 5, 2, 2, 4, 3, 5, 2, 5, 8, 4, 9, 0, 7, 7, 1, 1, 6, 7, 0,
            5, 5, 6, 0, 1, 3, 6, 0, 4, 8, 3, 9, 5, 8, 6, 4, 4, 6, 7, 0, 6, 3, 2, 4, 4, 1, 5, 7, 2,
            2, 1, 5, 5, 3, 9, 7, 5, 3, 6, 9, 7, 8, 1, 7, 9, 7, 7, 8, 4, 6, 1, 7, 4, 0, 6, 4, 9, 5,
            5, 1, 4, 9, 2, 9, 0, 8, 6, 2, 5, 6, 9, 3, 2, 1, 9, 7, 8, 4, 6, 8, 6, 2, 2, 4, 8, 2, 8,
            3, 9, 7, 2, 2, 4, 1, 3, 7, 5, 6, 5, 7, 0, 5, 6, 0, 5, 7, 4, 9, 0, 2, 6, 1, 4, 0, 7, 9,
            7, 2, 9, 6, 8, 6, 5, 2, 4, 1, 4, 5, 3, 5, 1, 0, 0, 4, 7, 4, 8, 2, 1, 6, 6, 3, 7, 0, 4,
            8, 4, 4, 0, 3, 1, 9, 9, 8, 9, 0, 0, 0, 8, 8, 9, 5, 2, 4, 3, 4, 5, 0, 6, 5, 8, 5, 4, 1,
            2, 2, 7, 5, 8, 8, 6, 6, 6, 8, 8, 1, 1, 6, 4, 2, 7, 1, 7, 1, 4, 7, 9, 9, 2, 4, 4, 4, 2,
            9, 2, 8, 2, 3, 0, 8, 6, 3, 4, 6, 5, 6, 7, 4, 8, 1, 3, 9, 1, 9, 1, 2, 3, 1, 6, 2, 8, 2,
            4, 5, 8, 6, 1, 7, 8, 6, 6, 4, 5, 8, 3, 5, 9, 1, 2, 4, 5, 6, 6, 5, 2, 9, 4, 7, 6, 5, 4,
            5, 6, 8, 2, 8, 4, 8, 9, 1, 2, 8, 8, 3, 1, 4, 2, 6, 0, 7, 6, 9, 0, 0, 4, 2, 2, 4, 2, 1,
            9, 0, 2, 2, 6, 7, 1, 0, 5, 5, 6, 2, 6, 3, 2, 1, 1, 1, 1, 1, 0, 9, 3, 7, 0, 5, 4, 4, 2,
            1, 7, 5, 0, 6, 9, 4, 1, 6, 5, 8, 9, 6, 0, 4, 0, 8, 0, 7, 1, 9, 8, 4, 0, 3, 8, 5, 0, 9,
            6, 2, 4, 5, 5, 4, 4, 4, 3, 6, 2, 9, 8, 1, 2, 3, 0, 9, 8, 7, 8, 7, 9, 9, 2, 7, 2, 4, 4,
            2, 8, 4, 9, 0, 9, 1, 8, 8, 8, 4, 5, 8, 0, 1, 5, 6, 1, 6, 6, 0, 9, 7, 9, 1, 9, 1, 3, 3,
            8, 7, 5, 4, 9, 9, 2, 0, 0, 5, 2, 4, 0, 6, 3, 6, 8, 9, 9, 1, 2, 5, 6, 0, 7, 1, 7, 6, 0,
            6, 0, 5, 8, 8, 6, 1, 1, 6, 4, 6, 7, 1, 0, 9, 4, 0, 5, 0, 7, 7, 5, 4, 1, 0, 0, 2, 2, 5,
            6, 9, 8, 3, 1, 5, 5, 2, 0, 0, 0, 5, 5, 9, 3, 5, 7, 2, 9, 7, 2, 5, 7, 1, 6, 3, 6, 2, 6,
            9, 5, 6, 1, 8, 8, 2, 6, 7, 0, 4, 2, 8, 2, 5, 2, 4, 8, 3, 6, 0, 0, 8, 2, 3, 2, 5, 7, 5,
            3, 0, 4, 2, 0, 7, 5, 2, 9, 6, 3, 4, 5, 0,
        ];

        let mut p: [u64; 988] = [1; 988];

        for i in 0..1000 {
            if i + 13 > 1000 {
                break;
            }

            let _ = &S[i..i + 13].iter().for_each(|f| {
                p[i] *= *f as u64;
            });
        }

        *p.iter().max().unwrap()
    }
}

struct P9;
impl Problem<u64> for P9 {
    fn title(&self) -> &'static str {
        "Special Pythagorean Triplet"
    }

    fn id(&self) -> &'static u16 {
        &9
    }

    fn answer(&self) -> &'static u64 {
        &0x1E65FB8
    }

    fn run(&self) -> u64 {
        const MAGIC: u64 = 1000;

        for a in 1..MAGIC {
            for b in a + 1..MAGIC {
                let c = ((a.pow(2) + b.pow(2)) as f64).sqrt();

                if c.fract() != 0.0 {
                    continue;
                }

                if a + b + c as u64 == MAGIC {
                    return a * b * c as u64;
                }
            }
        }

        0
    }
}

pub fn run() -> bool {
    for p in [
        P1.test(),
        P2.test(),
        P3.test(),
        P4.test(),
        P5.test(),
        P6.test(),
        P7.test(),
        P8.test(),
        P9.test(),
    ] {
        if !p {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        assert!(run())
    }
}
