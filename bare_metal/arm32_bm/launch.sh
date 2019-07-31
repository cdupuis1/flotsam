#!/bin/sh
qemu-system-arm -M versatilepb -m 256M -nographic -kernel test.bin
