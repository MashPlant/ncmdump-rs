fn main() {
  let emsdk = std::env::var("EMSDK").unwrap();
  println!("cargo:rerun-if-changed=src/add_tag.cpp");
  println!("cargo:rustc-flags=-L./taglib/build/taglib -l tag -L{0}/upstream/emscripten/system/local/lib -L{0}/upstream/emscripten/system/lib -L{0}/upstream/emscripten/cache/wasm -l c -l compiler_rt -l c++-noexcept -l c++abi-noexcept -l dlmalloc -l pthread_stub -l c_rt_wasm -l sockets", emsdk);
  cc::Build::new().file("src/add_tag.cpp").include("/usr/include/taglib").compile("add_tag");
}