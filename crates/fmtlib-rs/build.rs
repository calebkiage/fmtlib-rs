use std::{env, path::Path};

fn main() {
    // download
    let mut b = cxx_build::bridge("src/ffi.rs");
    if let Ok(tgt_env) = env::var("CARGO_CFG_TARGET_ENV") {
        if &tgt_env == "msvc" {
            b.flag_if_supported("/std:c++14");
            b.flag_if_supported("/EHsc");
        } else {
            b.flag_if_supported("-std=c++14");
        }
    }

    let cpp_src = Path::new("fmtlib-src");
    b.include(cpp_src.join("include"));

    b.file(cpp_src.join("src").join("format.cc"));
    b.compile("fmtlib"); // arbitrary library name, pick anything

    // Tell cargo to invalidate the built crate whenever the shim changes
    println!("cargo:rerun-if-changed=include/shim.h");
    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=fmtlib-src/src/format.cc");
}
