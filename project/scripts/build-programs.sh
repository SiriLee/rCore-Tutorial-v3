#!/usr/bin/env bash
# Cross-compile the Linux-ABI test programs as static RISC-V binaries.
#
# Output: project/build/min_raw, project/build/min_std
#
# Two constraints, both learned from the feasibility run:
#   - musl target: the kernel has no dynamic loader.
#   - rust-lld: the default linker is the host x86_64 linker and cannot link
#     RISC-V objects.
set -euo pipefail

cd "$(dirname "$0")/.."
TARGET=riscv64gc-unknown-linux-musl
OUT=build

mkdir -p "$OUT"

if ! rustup target list --installed | grep -q "$TARGET"; then
    echo "installing target $TARGET ..."
    rustup target add "$TARGET"
fi

rustc --target "$TARGET" -C linker=rust-lld -C panic=abort -C opt-level=2 \
      -C relocation-model=static -C link-arg=-static \
      -o "$OUT/min_raw" programs/min_raw.rs

rustc --target "$TARGET" -C linker=rust-lld -C opt-level=2 \
      -C target-feature=+crt-static \
      -o "$OUT/min_std" programs/min_std.rs

ls -l "$OUT/min_raw" "$OUT/min_std"
