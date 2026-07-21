use std::env;

fn main() {
    println!("cargo:rerun-if-changed=native/AppleSecureShareBridge.h");
    println!("cargo:rerun-if-changed=native/AppleSecureShareBridge.m");
    println!("cargo:rerun-if-changed=native/AppleSecureShareRecording.m");
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("macos") {
        return;
    }
    cc::Build::new()
        .file("native/AppleSecureShareBridge.m")
        .file("native/AppleSecureShareRecording.m")
        .flag("-fblocks")
        .flag("-fobjc-arc")
        .compile("dropsquash_apple_secure_share_bridge");
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=AVFoundation");
    println!("cargo:rustc-link-lib=framework=CoreMedia");
    println!("cargo:rustc-link-lib=framework=CoreVideo");
    println!("cargo:rustc-link-lib=framework=ScreenCaptureKit");
    println!("cargo:rustc-link-lib=framework=Vision");
}
