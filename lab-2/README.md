# Lab 2 起始覆盖层

本目录不是独立工程。先复制已完成的 `lab-1/` 为 `lab-2/`，再把本目录的内容覆盖到
`lab-2/`。覆盖层新增 `src/mm/` 骨架、把双 hart 启动栈扩展到每 hart 16 KiB，并提供
Lab 2 验收脚本；它不包含页帧分配、页表遍历和内核地址空间的最终实现。

覆盖后的第一次 `make build` 仍使用 Lab 1 的 `src/main.rs`，尚不会编译新增的 `src/mm/`；
按讲义加入 `mod mm;` 后，才会检查内存模块代码。工程继续沿用 Lab 1 的 crate 名
`lab1-rcore`，因为本覆盖层的 Makefile 按该名称寻找 ELF，请不要只改 Cargo 包名。

按材料中的 `lab-2.md` 依次完成 TODO。最终执行 `rg -n TODO src/mm src/main.rs` 应无输出，
`make check` 应以 `lab-2 check: PASS` 结束。

OpenSpec 规格位于课程材料 `lab-2/lab-2-memory.spec.md`，按主讲义安装到仓库根目录的
`openspec/specs/lab-2-memory/spec.md`，不放入本覆盖层。
