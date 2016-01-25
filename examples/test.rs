extern crate cffi;

fn main() {
    println!("7 is prime == {}", cffi::is_prime(7));
    println!("7 is prime == {}", cffi::foreign_is_prime(7));
}
