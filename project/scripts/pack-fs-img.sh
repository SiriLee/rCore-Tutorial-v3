#!/usr/bin/env bash
# Build rCore, inject the Linux programs into the easy-fs image, and repack it.
#
# easy-fs-fuse enumerates *.rs names in the source dir and reads the matching
# binary from the target dir. A program is therefore staged by placing a
# same-named .rs placeholder next to its binary. Staging happens under
# project/build/ so the upstream user/ directory is never modified.
set -euo pipefail

cd "$(dirname "$0")/../.."
PROJECT=project
TARGET=riscv64gc-unknown-none-elf
USER_BIN="user/target/$TARGET/release"
STAGE="$PROJECT/build/fs-stage"

make -C os build

rm -rf "$STAGE"
mkdir -p "$STAGE/src" "$STAGE/bin"

cp user/src/bin/*.rs "$STAGE/src/"
find "$USER_BIN" -maxdepth 1 -type f ! -name fs.img -exec cp {} "$STAGE/bin/" \;

stage_program() {
    local name="$1" src="$2"
    [ -f "$src" ] || { echo "missing $src" >&2; exit 1; }
    cp "$src" "$STAGE/bin/$name"
    : > "$STAGE/src/$name.rs"
    echo "staged: $name"
}

stage_program min_raw  "$PROJECT/build/min_raw"
stage_program min_std  "$PROJECT/build/min_std"

# Optional third-party programs (fetched manually, see third_party/README.md).
if [ -f "$PROJECT/third_party/bin/busybox" ]; then
    stage_program busybox "$PROJECT/third_party/bin/busybox"
fi

(cd easy-fs-fuse && cargo run --release -- -s "../$STAGE/src/" -t "../$STAGE/bin/")
cp "$STAGE/bin/fs.img" "$USER_BIN/fs.img"
echo "fs image ready: $USER_BIN/fs.img"
