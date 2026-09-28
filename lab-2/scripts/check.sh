#!/bin/sh
set -eu

kernel=${1:-kernel-qemu.elf}
qemu=${QEMU:-qemu-system-riscv64}
objdump=${OBJDUMP:-rust-objdump}
qemu_log=$(mktemp -t lab2-qemu.XXXXXX)
asm_log=$(mktemp -t lab2-asm.XXXXXX)
qemu_pid=

cleanup() {
    if [ -n "$qemu_pid" ] && kill -0 "$qemu_pid" 2>/dev/null; then
        kill "$qemu_pid" 2>/dev/null || true
        wait "$qemu_pid" 2>/dev/null || true
    fi
    rm -f "$qemu_log" "$asm_log"
}
trap cleanup EXIT INT TERM

fail_with_log() {
    echo "$1" >&2
    echo 'QEMU output:' >&2
    sed -n '1,160p' "$qemu_log" >&2
    exit 1
}

require_output() {
    grep -F "$1" "$qemu_log" >/dev/null 2>&1 \
        || fail_with_log "missing expected output: $1"
}

"$objdump" -d "$kernel" >"$asm_log"
grep -E '(csrw|csrrw).*satp' "$asm_log" >/dev/null \
    || fail_with_log 'missing satp write in kernel disassembly'
grep -E '[[:space:]]sfence\.vma' "$asm_log" >/dev/null \
    || fail_with_log 'missing sfence.vma in kernel disassembly'

"$qemu" -machine virt -bios none -kernel "$kernel" -m 128M -smp 2 -nographic \
    >"$qemu_log" 2>&1 &
qemu_pid=$!

attempt=0
while [ "$attempt" -lt 150 ]; do
    if grep -F 'hart 0 paging enabled' "$qemu_log" >/dev/null 2>&1 && \
       grep -F 'hart 1 paging enabled' "$qemu_log" >/dev/null 2>&1; then
        break
    fi
    if ! kill -0 "$qemu_pid" 2>/dev/null; then
        echo 'QEMU exited before producing the expected output:' >&2
        sed -n '1,160p' "$qemu_log" >&2
        exit 1
    fi
    attempt=$((attempt + 1))
    sleep 0.1
done

require_output 'lab-2 address test: PASS'
require_output 'lab-2 frame test: PASS'
require_output 'lab-2 page-table test: PASS'
require_output 'lab-2 kernel-space test: PASS'
require_output 'hart 0 paging enabled'
require_output 'hart 1 paging enabled'

sed -n '1,160p' "$qemu_log"
echo 'lab-2 check: PASS'
