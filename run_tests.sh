#!/bin/bash
set -e

export CARGO_MANIFEST_DIR=$(pwd)
cargo bootimage --target x86_64-unit_os.json

QEMU_BIN="${{ steps.qemu-setup.outputs.qemu-system-x86_64 }}"
if [ -z "$QEMU_BIN" ]; then
  echo "Error: QEMU executable path not found."
  exit 1
fi

"$QEMU_BIN" \
  -drive format=raw,file=target/x86_64-unit_os/debug/bootimage-rusty_os.bin \
  -no-reboot \
  -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
  -serial stdio \
  -display none
