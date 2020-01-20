use std::env;
use std::path::PathBuf;

fn main() {
  println!("cargo:rustc-link-lib=onigmo");
  let bindgens = bindgen::Builder::default()
    .header("wrapper.h")
    .generate()
    .expect("Unable to generate bindgen");

  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindgens
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Coludn't write bindgens!");
}
