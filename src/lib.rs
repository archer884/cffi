#![feature(step_by, test)]

extern crate libc;
extern crate test;

#[cfg(test)]
extern crate primal;

use libc::{c_int, c_ulonglong};

extern {
    fn c_is_prime(n: c_ulonglong) -> c_int;
}

pub fn foreign_is_prime(n: u64) -> c_int {
    unsafe {
        c_is_prime(n)
    }
}

pub fn is_prime(n: u64) -> bool {
    match n {
        0 | 1 => false,
        2 | 3 => true,
        n if n & 1 != 1 => false,
        n => {
            for i in (3..).step_by(2).take_while(|i| i * i <= n) {
                if n % i == 0 {
                    return false
                }
            }
            return true
        }
    }
}

pub fn is_prime_foreign_clone(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    for i in (2..).take_while(|i| i * i <= n) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use primal::Primes;
    use super::{foreign_is_prime, is_prime, is_prime_foreign_clone};
    use test;

    #[bench]
    fn bench_native(b: &mut test::Bencher) {
        b.iter(|| for n in 1000000..1001000 {
            test::black_box(is_prime(n));
        })
    }

    #[bench]
    fn bench_foreign_like_native(b: &mut test::Bencher) {
        b.iter(|| for n in 1000000..1001000 {
            test::black_box(is_prime_foreign_clone(n));
        })
    }

    #[bench]
    fn bench_foreign(b: &mut test::Bencher) {
        b.iter(|| for n in 1000000..1001000 {
            test::black_box(foreign_is_prime(n));
        })
    }

    // I believe it's reasonable to ask me to demonstrate that my implementation of this prime
    // function works, because... Yeah. I don't have that much confidence in myself, honestly. :)
    #[test]
    fn native_works() {
        let first_thousand_primes: Vec<_> = Primes::all().take(1000).map(|n| n as u64).collect();
        let x = find_first_thousand_primes();

        assert_eq!(&first_thousand_primes, &x);
    }

    fn find_first_thousand_primes() -> Vec<u64> {
        (1..).filter(|&n| is_prime(n)).take(1000).collect()
    }
}
