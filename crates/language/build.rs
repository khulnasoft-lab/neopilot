fn main() {
    if let Ok(bundled) = std::env::var("NEOPILOT_BUNDLE") {
        println!("cargo:rustc-env=NEOPILOT_BUNDLE={}", bundled);
    }
}
