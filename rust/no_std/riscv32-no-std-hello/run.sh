#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

echo "Building (release)..."
cargo build --release

ELF="target/riscv32imac-unknown-none-elf/release/riscv32-bare-metal"

echo "Launching QEMU..."
qemu-system-riscv32 \
    -machine virt \
    -nographic \
    -bios none \
    -kernel "$ELF"
