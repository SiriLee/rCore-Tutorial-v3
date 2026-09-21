# Lab 1 起始工程

本目录随实验材料提供，不依赖另行发放的课程源码。先按课程材料中的 `lab-1.md` 任务 1
复制工程，再完成源码中标有 `TODO(task 4)`、`TODO(task 5)`、`TODO(task 6)` 和
`TODO(tasks 5-7)` 的部分。

起始工程应能执行 `make build`，但不会输出内容，也不会通过 `make check`。完成 Lab 1
后，`rg -n TODO src` 应无输出，`make check` 应以 `lab-1 check: PASS` 结束。

OpenSpec 规格位于课程材料 `lab-1/lab-1-machine-boot.spec.md`，应按主讲义复制到
`rCore-Tutorial-v3/openspec/specs/lab-1-machine-boot/spec.md`，不放在本 Cargo 工程中。
