#!/usr/bin/env bash
# Boot rCore in QEMU with the serial console on stdout/stdin.
# Exit: press Ctrl-A, release, then X.
set -euo pipefail

cd "$(dirname "$0")/../.."

KERNEL=os/target/riscv64gc-unknown-none-elf/release/os.bin
FS=user/target/riscv64gc-unknown-none-elf/release/fs.img
BOOTLOADER=bootloader/rustsbi-qemu.bin

for f in "$KERNEL" "$FS" "$BOOTLOADER"; do
    [ -f "$f" ] || { echo "missing $f -- run scripts/pack-fs-img.sh first" >&2; exit 1; }
done

exec qemu-system-riscv64 \
    -machine virt \
    -bios "$BOOTLOADER" \
    -serial stdio \
    -display none \
    -device loader,file="$KERNEL",addr=0x80200000 \
    -drive file="$FS",if=none,format=raw,id=x0 \
    -device virtio-blk-device,drive=x0 \
    -device virtio-gpu-device \
    -device virtio-keyboard-device \
    -device virtio-mouse-device \
    -device virtio-net-device,netdev=net0 \
    -netdev user,id=net0,hostfwd=udp::6200-:2000,hostfwd=tcp::6201-:80
