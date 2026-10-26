#!/bin/sh
set -eu

kernel=${1:-kernel-qemu.elf}
qemu=${QEMU:-qemu-system-riscv64}
objdump=${OBJDUMP:-rust-objdump}
qemu_log=$(mktemp -t lab3-qemu.XXXXXX)
asm_log=$(mktemp -t lab3-asm.XXXXXX)
input_fifo=$(mktemp -u "${TMPDIR:-/tmp}/lab3-input.XXXXXX")
qemu_pid=
writer_pid=

cleanup() {
    if [ -n "$writer_pid" ] && kill -0 "$writer_pid" 2>/dev/null; then
        kill "$writer_pid" 2>/dev/null || true
    fi
    if [ -n "$qemu_pid" ] && kill -0 "$qemu_pid" 2>/dev/null; then
        kill "$qemu_pid" 2>/dev/null || true
        wait "$qemu_pid" 2>/dev/null || true
    fi
    rm -f "$qemu_log" "$asm_log" "$input_fifo"
}
trap cleanup EXIT INT TERM

fail_with_log() {
    echo "$1" >&2
    echo 'QEMU output:' >&2
    sed -n '1,200p' "$qemu_log" >&2
    exit 1
}

require_output() {
    grep -F "$1" "$qemu_log" >/dev/null 2>&1 \
        || fail_with_log "missing expected output: $1"
}

"$objdump" -d "$kernel" >"$asm_log"
grep -E '[[:space:]]csr[a-z]*[[:space:]]+[^#]*stvec' "$asm_log" >/dev/null \
    || fail_with_log 'missing stvec CSR instruction'
grep -E '[[:space:]]csr[a-z]*[[:space:]]+[^#]*mtvec' "$asm_log" >/dev/null \
    || fail_with_log 'missing mtvec CSR instruction'
grep -E '[[:space:]]csr[a-z]*[[:space:]]+[^#]*mscratch' "$asm_log" >/dev/null \
    || fail_with_log 'missing mscratch CSR instruction'
grep -E '[[:space:]]csr[a-z]*[[:space:]]+[^#]*sip' "$asm_log" >/dev/null \
    || fail_with_log 'missing sip CSR instruction'
grep -E '[[:space:]]sret([[:space:]]|$)' "$asm_log" >/dev/null \
    || fail_with_log 'missing sret'
grep -E '[[:space:]]mret([[:space:]]|$)' "$asm_log" >/dev/null \
    || fail_with_log 'missing mret'
grep -F '<kernel_vector>:' "$asm_log" >/dev/null \
    || fail_with_log 'missing kernel_vector'
grep -F '<timer_vector>:' "$asm_log" >/dev/null \
    || fail_with_log 'missing timer_vector'

mkfifo "$input_fifo"
(sleep 1; printf 'K'; sleep 3) >"$input_fifo" &
writer_pid=$!
"$qemu" -machine virt -bios none -kernel "$kernel" -m 128M -smp 2 \
    -nographic <"$input_fifo" >"$qemu_log" 2>&1 &
qemu_pid=$!

attempt=0
while [ "$attempt" -lt 200 ]; do
    if grep -F 'lab-3 timer test: PASS' "$qemu_log" >/dev/null 2>&1 && \
       grep -F 'lab-3 uart test: PASS byte=K' "$qemu_log" >/dev/null 2>&1; then
        break
    fi
    if ! kill -0 "$qemu_pid" 2>/dev/null; then
        fail_with_log 'QEMU exited before Lab 3 completed'
    fi
    attempt=$((attempt + 1))
    sleep 0.1
done

require_output 'lab-3 context test: PASS'
require_output 'hart 0 traps enabled'
require_output 'hart 1 traps enabled'
require_output 'lab-3 timer test: PASS'
require_output 'lab-3 uart test: PASS byte=K'

sed -n '1,200p' "$qemu_log"
echo 'lab-3 check: PASS'
