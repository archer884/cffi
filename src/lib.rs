#![feature(step_by, test)]

extern crate libc;
extern crate test;

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
        n if n & 1 == 1 => false,
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
}
