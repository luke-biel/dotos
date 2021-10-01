use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8_lossy(&output.stdout);
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    let output = Command::new("date").output().unwrap();
    let date = String::from_utf8_lossy(&output.stdout);
    println!("cargo:rustc-env=BUILD_DATE={}", date);
    if let Some(log_level) = option_env!("LOG_LEVEL") {
        println!("cargo:rustc-env=LOG_LEVEL={}", log_level);
    }
}
