use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    if target.starts_with("thumbv") {
        fs::copy(
            format!("bin/{}.a", target),
            out_dir.join("liblpc54608-pac.a"),
        )
        .unwrap();
        println!("cargo:rustc-link-lib=static=lpc54608-pac");
    }

    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        File::create(out.join("device.x"))
            .unwrap()
            .write_all(include_bytes!("device.x"))
            .unwrap();
        println!("cargo:rustc-link-search={}", out.display());
        println!("cargo:rerun-if-changed=device.x");
        println!("cargo:rerun-if-changed=startup.s");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
