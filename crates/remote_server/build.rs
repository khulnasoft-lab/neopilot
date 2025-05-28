use std::process::Command;

const NEOPILOT_MANIFEST: &str = include_str!("../neopilot/Cargo.toml");

fn main() {
    let neopilot_cargo_toml: cargo_toml::Manifest =
        toml::from_str(NEOPILOT_MANIFEST).expect("failed to parse neopilot Cargo.toml");
    println!(
        "cargo:rustc-env=NEOPILOT_PKG_VERSION={}",
        neopilot_cargo_toml.package.unwrap().version.unwrap()
    );
    println!(
        "cargo:rustc-env=TARGET={}",
        std::env::var("TARGET").unwrap()
    );

    // Populate git sha environment variable if git is available
    println!("cargo:rerun-if-changed=../../.git/logs/HEAD");
    if let Some(output) = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|output| output.status.success())
    {
        let git_sha = String::from_utf8_lossy(&output.stdout);
        let git_sha = git_sha.trim();

        println!("cargo:rustc-env=NEOPILOT_COMMIT_SHA={git_sha}");
    }
}
