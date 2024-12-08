extern crate cmake;

use cmake::Config;

fn main() {
    let dst = Config::new("libdepth").build();

    println!("cargo:rustc-link-search=native={}", dst.display());

    println!("cargo:rustc-link-lib=static=depth");

    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=CoreML");

    println!("cargo:rerun-if-changed=libdepth");
}
