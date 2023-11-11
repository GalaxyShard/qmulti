use std::env;
use std::path::PathBuf;

fn check_env(env: &str, value: &str) -> bool {
    env::var(env).as_deref() == Ok(value)
}
fn main() -> Result<(), ()> {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut bonjour = cfg!(zeroconf_impl="bonjour");
    let mut avahi = cfg!(zeroconf_impl="avahi");
    let mut android_nsd = cfg!(zeroconf_impl="android_nsd");
    let windns = cfg!(zeroconf_impl="windns");

    if !bonjour && !avahi && !android_nsd && !windns {
        bonjour = check_env("CARGO_CFG_TARGET_VENDOR", "apple")
            || check_env("CARGO_CFG_TARGET_OS", "windows");

        avahi = check_env("CARGO_CFG_TARGET_OS", "linux");
        android_nsd = check_env("CARGO_CFG_TARGET_OS", "android");

        // note: default to bonjour on windows (for now)
        // windns = check_env("CARGO_CFG_TARGET_OS", "windows");

        if bonjour {
            println!("cargo:rustc-cfg=zeroconf_impl=\"bonjour\"");
        } else if avahi {
            println!("cargo:rustc-cfg=zeroconf_impl=\"avahi\"");
        } else if android_nsd {
            println!("cargo:rustc-cfg=zeroconf_impl=\"android_nsd\"");
        } else if windns {
            println!("cargo:rustc-cfg=zeroconf_impl=\"windns\"");
        }
    }

    if bonjour {
        // println!("cargo:rerun-if-changed=dnssd-wrapper.h");

        if check_env("CARGO_CFG_TARGET_OS", "linux") {
            println!("cargo:rustc-link-lib=dylib=avahi-compat-libdns_sd");
        }

        // bindgen::Builder::default()
        //     .header("dnssd-wrapper.h")
        //     .blocklist_function("strtold") // not ffi safe
        //     // Tell cargo to invalidate the built crate whenever any of the included header files changed.
        //     .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        //     .generate()
        //     .expect("Unable to generate bindings")
        //     .write_to_file(out_path.join("dnssd-bindings.rs"))
        //     .expect("Couldn't write bindings!");
    } else if avahi {
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
            .write_to_file(out_path.join("avahi-bindings.rs"))
            .expect("Couldn't write bindings!");
    } else if android_nsd {
    } else if windns {
    }
    Ok(())
}