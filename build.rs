extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=lib/f4se/build/f4se/f4se_common/Release/f4se_common");

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::builder()
        .clang_args(&["-x", "c++"])
        .clang_args(&["-Ilib/common", "-Ilib/f4se"])
        .header("wrapper.h")
        .allowlist_type("F4SEInterface")
        .allowlist_type("PluginInfo")
        .allowlist_var("RelocationManager_s_baseAddr")
        .generate()
        .expect("Unable to generate bindings");

    let out_dir = env::var("OUT_DIR").unwrap();
    let output = PathBuf::from(out_dir).join("bindings.rs");

    bindings
        .write_to_file(output)
        .expect("Couldn't write bindings!");
}
