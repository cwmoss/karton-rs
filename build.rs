use std::process::Command;

fn main() {
    // Get the current date in YYYYMMDD format
    let output = Command::new("date")
        .args(&["+%Y%m%d"])
        .output()
        .expect("Failed to execute date command");

    let build_time = String::from_utf8(output.stdout)
        .expect("Invalid UTF-8 output")
        .trim()
        .to_string();

    println!("cargo:rustc-env=BUILD_TIME={}", build_time);
}