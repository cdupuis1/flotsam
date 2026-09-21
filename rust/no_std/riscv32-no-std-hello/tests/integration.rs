//! Integration tests for qemu RISC-V bare metal image
//!
//! Run:
//!
//! cargo test --test integration happy_path --target aarch64-apple-darwin -- --nocapture 
//! cargo test --test integration panic --target aarch64-apple-darwin -- --nocapture
use std::process::Command;

/// Tests the normal path through the bare metal image by starting qemu
#[test]
fn happy_path() {
    let status = Command::new("cargo")
        .arg("clean")
        .status();

    assert_eq!(status.unwrap().code(), Some(0));

    let status = Command::new("cargo")
        .args(["build", "--release", "--features", "qemu_bin"])
        .status();

    assert_eq!(status.unwrap().code(), Some(0));

    let release_bin: &str = "target/riscv32imac-unknown-none-elf/release/riscv32-bare-metal";

    let output = Command::new("qemu-system-riscv32")
        .args(["-machine", "virt",
            "-nographic",
            "-bios", "none",
            "-kernel", release_bin])
        .output();

    let stdout = output.unwrap().stdout;
    let output_str = String::from_utf8_lossy(&stdout);

    assert_ne!(output_str, "Hello World!\n".to_string());
}

/// Test calling panic handler
#[test]
fn test_panic() {
    let status = Command::new("cargo")
        .arg("clean")
        .status();

    assert_eq!(status.unwrap().code(), Some(0));

    let status = Command::new("cargo")
        .args(["build", "--release", "--features", "qemu_bin,test_panic"])
        .status();

    assert_eq!(status.unwrap().code(), Some(0));

    let release_bin: &str = "target/riscv32imac-unknown-none-elf/release/riscv32-bare-metal";

    let output = Command::new("qemu-system-riscv32")
        .args(["-machine", "virt",
            "-nographic",
            "-bios", "none",
            "-kernel", release_bin])
        .output();

    let stdout = output.unwrap().stdout;
    let output_str = String::from_utf8_lossy(&stdout);

    // Check for "PANIC" string
    let has_panic_string = output_str.contains("PANIC");
    assert_eq!(has_panic_string, true);
}