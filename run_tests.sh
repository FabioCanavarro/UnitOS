#!/bin/bash
set -e

export CARGO_MANIFEST_DIR=$(pwd)
cargo bootimage --target x86_64-unit_os.json

if [ -z "$QEMU_BIN" ]; then
  echo "Error: QEMU_BIN environment variable not set."
  exit 1
fi

"$QEMU_BIN" \
  -drive format=raw,file=target/x86_64-unit_os/debug/bootimage-rusty_os.bin \
  -no-reboot \
  -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
  -serial stdio \
  -display none
