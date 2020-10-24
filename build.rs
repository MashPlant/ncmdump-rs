fn main() {
  println!("cargo:rerun-if-changed=src/add_tag.cpp");
  println!("cargo:rustc-flags=-l tag -l stdc++");
  cc::Build::new().file("src/add_tag.cpp").compile("add_tag");
}