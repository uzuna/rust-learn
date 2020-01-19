extern crate cc;

fn main() {
  cc::Build::new().file("c_src/fib.c").compile("fib");
  cc::Build::new()
    .file("c_src/ownership.c")
    .compile("ownership");
}
