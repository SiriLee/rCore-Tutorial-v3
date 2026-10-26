# Lab 3 起始覆盖层

本目录不是独立工程。先复制已完成的 `lab-2/` 为 `lab-3/`，再把本目录内容覆盖到
`lab-3/`。覆盖层加入 trap 汇编、PLIC、时钟和验收脚本；已有的 Lab 1/2 源码不会被删除。

```bash
cp -R lab-2 lab-3
cp -R "$OSLAB_MATERIALS/starter/." lab-3/
chmod +x lab-3/scripts/check.sh
```

覆盖后的第一次 `make build` 仍使用 Lab 2 的 `src/main.rs`，尚不会编译新增模块；按
讲义加入 `mod plic/mod timer/mod trap` 后，才会检查三个 Rust 骨架。工程继续沿用
`lab1-rcore` 作为 crate 和 ELF 名称，请不要只修改 Cargo 包名。

`src/mm/memory_set.rs` 不在覆盖层中，仍来自 Lab 2；请按讲义任务 2 在该文件中增加
PLIC 三页映射和权限检查。这样可以复用已验证的页表接口，不需要复制或新建内存管理
文件。

按材料中的 `lab-3.md` 依次完成 TODO。最终执行 `rg -n TODO src` 应无输出，
`make check` 应以 `lab-3 check: PASS` 结束。

OpenSpec 规格位于课程材料 `lab-3/lab-3-trap.spec.md`，按主讲义安装到仓库根目录的
`openspec/specs/lab-3-trap-interrupt/spec.md`，不放入本覆盖层。
