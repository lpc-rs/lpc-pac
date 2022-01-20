use std::env;
use std::fs;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());
        let device_file = if env::var_os("CARGO_FEATURE_LPC54605").is_some() {
            "src/lpc54605/device.x"
        } else if env::var_os("CARGO_FEATURE_LPC54606").is_some() {
            "src/lpc54606/device.x"
        } else if env::var_os("CARGO_FEATURE_LPC54607").is_some() {
            "src/lpc54607/device.x"
        } else if env::var_os("CARGO_FEATURE_LPC54608").is_some() {
            "src/lpc54608/device.x"
        } else if env::var_os("CARGO_FEATURE_LPC54616").is_some() {
            "src/lpc54616/device.x"
        } else if env::var_os("CARGO_FEATURE_LPC54618").is_some() {
            "src/lpc54618/device.x"
        } else if env::var_os("CARGO_FEATURE_LPC54628").is_some() {
            "src/lpc54628/device.x"
        } else {
            panic!("No device features selected");
        };
        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }
    println!("cargo:rerun-if-changed=build.rs");
}
