#!/bin/sh
#
# launch.sh
#
# qemu command line to launch virtual machine
qemu-system-aarch64 -nographic -machine virt -m 512 -smp 1 -cpu cortex-a57 -kernel main.bin
