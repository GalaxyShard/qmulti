use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    
    if env::var("CARGO_CFG_TARGET_VENDOR").as_deref() == Ok("apple")
        || env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows")
    {
        println!("cargo:rerun-if-changed=dnssd-wrapper.h");

        bindgen::Builder::default()
            .header("dnssd-wrapper.h")
            .blocklist_function("strtold") // not ffi safe
            // Tell cargo to invalidate the built crate whenever any of the included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file(out_path)
            .expect("Couldn't write bindings!");
    } else if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("linux") {
        println!("cargo:rerun-if-changed=avahi-wrapper.h");

        // Explicitly link Avahi dynamically
        println!("cargo:rustc-link-lib=dylib=avahi-client");
        println!("cargo:rustc-link-lib=dylib=avahi-common");

        bindgen::Builder::default()
            .header("avahi-wrapper.h")
            // Tell cargo to invalidate the built crate whenever any of the included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file(out_path)
            .expect("Couldn't write bindings!");
    }
}
