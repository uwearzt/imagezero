// ------------------------------------------------------------------------------
// Copyright 2021 Uwe Arzt, mail@uwe-arzt.de
// SPDX-License-Identifier: Apache-2.0
// ------------------------------------------------------------------------------

extern crate bindgen;

use std::env;
use std::path::PathBuf;

use cmake::Config;

fn main() {
    // build the library
    let dst = Config::new("imagezero-cpp")
        .profile("Release")
        .no_build_target(true)
        .build();

    // Creste bindings
    //println!("cargo:rustc-link-search=all={}", dst.display());
    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=dylib=imagezero");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.hpp");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        .clang_arg("-I/Library/Developer/CommandLineTools/usr/include/c++/v1")
        // .clang_arg("-I/Library/Developer/CommandLineTools/usr/lib/clang/12.0.0/include")
        .clang_arg("-Iimagezero-cpp/include")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
