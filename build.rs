use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .expect("Could not execute git");

    let hash = String::from_utf8(output.stdout)
        .expect("Git hash was not valid UTF-8");

    println!("cargo::rustc-env=BUILD_GIT_HASH={}", hash.trim());
}