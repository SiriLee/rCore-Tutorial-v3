KERNEL := target/riscv64gc-unknown-none-elf/release/lab1-rcore
QEMU ?= qemu-system-riscv64
OBJDUMP ?= rust-objdump

.PHONY: all build qemu disasm check clean

all: build

build:
	cargo build --release
	cp $(KERNEL) kernel-qemu.elf

qemu: build
	$(QEMU) -machine virt -bios none -kernel kernel-qemu.elf -m 128M -smp 2 -nographic

disasm: build
	$(OBJDUMP) -S kernel-qemu.elf > kernel.asm

check: build
	QEMU="$(QEMU)" OBJDUMP="$(OBJDUMP)" ./scripts/check.sh kernel-qemu.elf

clean:
	cargo clean
	rm -f kernel-qemu.elf kernel.asm
