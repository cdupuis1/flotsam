#!/bin/sh
#
# QEMU aarch64 startup command
#
qemu-system-aarch64 -hda linux.img -nographic -net nic -net user,hostfwd=tcp::8000-:22 -pflash QEMU_EFI.img -machine virt -cpu cortex-a57 -m 2048 -smp 3 -boot d
