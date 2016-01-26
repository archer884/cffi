extern crate cffi;

trait Boolish {
    fn as_bool(&self) -> bool;
}

impl Boolish for i32 {
    fn as_bool(&self) -> bool {
        *self != 0
    }
}

impl Boolish for bool {
    fn as_bool(&self) -> bool {
        *self
    }
}

fn main() {
    println!("{}", build_result(7, cffi::is_prime));
    println!("{}", build_result(7, cffi::foreign_is_prime));
    println!("{}", build_result(7, cffi::is_prime_foreign_clone));
}

fn build_result<F, R>(candidate: u64, f: F) -> String
    where R: Boolish,
          F: FnOnce(u64) -> R,
{
    if f(candidate).as_bool() {
        format!("{} is prime", candidate)
    } else {
        format!("{} is not prime", candidate)
    }
}
