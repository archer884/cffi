extern crate gcc;

fn main() {
    gcc::Config::new()
        .file("src/prime.c")
        .compile("libprime.a");
}
