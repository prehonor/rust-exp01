
extern crate bindgen;

use std::env;
// use bindgen::builder;
use std::path::PathBuf;

fn main() {
    let lib_path = PathBuf::from(env::current_dir().unwrap().join("cglib"));

    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=static=bookmarks");
    // println!("cargo:rustc-link-lib=static=others");  // because of multiple definition of `Hello::print()'
    // println!("cargo:rustc-link-arg=-Wl,-rpath={}", lib_path.display());  //workds in nighly
    // println!("cargo:rustc-env=LD_LIBRARY_PATH={}", lib_path.display());  //always works

    let bindings = bindgen::Builder::default()
        .header("cglib/library.h")
        .allowlist_function("hello")
        .generate().unwrap();

    bindings.write_to_file("src/cheader.rs").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}