extern crate onigmo as onig;

fn main() {
  let mut reg: onig::Regex = onig::Regex::new("a(.*)b|[e-f]+").unwrap();
  let s = "zzzzzaffffffb";
  match reg.search(s) {
    Some(ret) => {
      use std::str::from_utf8;
      for (beg, end) in ret.position() {
        println!("{}", from_utf8(&s.as_bytes()[beg..end]).unwrap());
      }
    }
    None => println!("not match"),
  }
}
