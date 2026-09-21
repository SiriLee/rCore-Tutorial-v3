#!/bin/sh
set -eu

kernel=${1:-kernel-qemu.elf}
qemu=${QEMU:-qemu-system-riscv64}
objdump=${OBJDUMP:-rust-objdump}
qemu_log=$(mktemp -t lab1-qemu.XXXXXX)
elf_log=$(mktemp -t lab1-elf.XXXXXX)
asm_log=$(mktemp -t lab1-asm.XXXXXX)
qemu_pid=

cleanup() {
    if [ -n "$qemu_pid" ] && kill -0 "$qemu_pid" 2>/dev/null; then
        kill "$qemu_pid" 2>/dev/null || true
        wait "$qemu_pid" 2>/dev/null || true
    fi
    rm -f "$qemu_log" "$elf_log" "$asm_log"
}
trap cleanup EXIT INT TERM

"$objdump" -f "$kernel" >"$elf_log"
grep -F 'architecture: riscv64' "$elf_log" >/dev/null
grep -F 'start address: 0x0000000080000000' "$elf_log" >/dev/null

"$objdump" -d "$kernel" >"$asm_log"
grep -E '[[:space:]]mret([[:space:]]|$)' "$asm_log" >/dev/null
grep -E '[[:space:]]amo[a-z.]*[[:space:]]' "$asm_log" >/dev/null
grep -E '[[:space:]]fence([[:space:]]|\.)' "$asm_log" >/dev/null
grep -E '(csrr|csrc|csrs).*sstatus' "$asm_log" >/dev/null

"$qemu" -machine virt -bios none -kernel "$kernel" -m 128M -smp 2 -nographic \
    >"$qemu_log" 2>&1 &
qemu_pid=$!

attempt=0
while [ "$attempt" -lt 150 ]; do
    if grep -F 'hart 0 says:' "$qemu_log" >/dev/null 2>&1 && \
       grep -F 'hart 1 says:' "$qemu_log" >/dev/null 2>&1; then
        break
    fi
    if ! kill -0 "$qemu_pid" 2>/dev/null; then
        echo 'QEMU exited before producing the expected output:' >&2
        sed -n '1,120p' "$qemu_log" >&2
        exit 1
    fi
    attempt=$((attempt + 1))
    sleep 0.1
done

grep -F 'ECNU OSLab rCore kernel entered S-mode' "$qemu_log" >/dev/null
grep -F 'interrupt nesting self-test: PASS' "$qemu_log" >/dev/null
grep -F 'Rust formatting: char=A string=uart d=-2025 p=2025 x=0x1234567880000000' "$qemu_log" >/dev/null
grep -F 'hart 0 says: char=A string=uart d=-2025 p=2025 x=0x1234567880000000' "$qemu_log" >/dev/null
grep -F 'hart 1 says: char=B string=uart d=-2025 p=2025 x=0x1234567880000000' "$qemu_log" >/dev/null

sed -n '1,120p' "$qemu_log"
echo 'lab-1 check: PASS'
