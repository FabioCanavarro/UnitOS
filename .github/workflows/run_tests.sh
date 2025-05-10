#!/bin/bash
set -e

cargo bootimage --target x86_64-unit_os.json

qemu-system-x86_64 \
    -drive format=raw,file=target/x86_64-unit_os/debug/bootimage-rusty_os.bin \
    -no-reboot \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
    -serial stdio \
    -display none
