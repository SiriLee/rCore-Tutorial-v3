# Lab 1 Machine Boot Specification

## Purpose

本规格定义 Lab 1 裸机内核的双 hart 启动、独立启动栈、M-mode 到 S-mode 的特权级切换、UART 初始化、格式化串口输出、并发互斥、初始化发布和自动验收行为，并给出可以由 ELF、反汇编和 QEMU 输出观察的通过条件。

## Requirements

### Requirement: 内核从规定地址启动

构建系统 SHALL 生成 RISC-V 64 位静态 ELF，其入口符号为 `_entry`，入口地址为 `0x80000000`。

#### Scenario: 检查 ELF 入口

- **WHEN** 使用 RISC-V `objdump` 检查构建产物
- **THEN** 产物架构为 RISC-V 64 位且入口地址等于 `0x80000000`

### Requirement: 双 hart 使用独立启动栈

内核 MUST 为两个 hart 各分配一个不重叠的 4 KiB 栈，并在调用 Rust `start` 前设置栈指针。

#### Scenario: 双 hart 进入 Rust 启动函数

- **WHEN** QEMU 以 `-smp 2` 和 `-bios none` 启动内核
- **THEN** 两个 hart 均能在不破坏对方调用栈的情况下进入 Rust `start`

### Requirement: 内核进入 S-mode

内核 SHALL 在分页关闭时配置必要的 CSR 和 PMP，将 hart ID 保存到 `tp`，并通过 `mret` 从 M-mode 进入 `rust_main`。

#### Scenario: 双 hart 完成特权级切换

- **WHEN** 两个 hart 分别执行 Rust `start`
- **THEN** 两者均从 S-mode 执行 `rust_main`，且能从 `tp` 读取 hart ID

### Requirement: UART 提供字符输出

内核 SHALL 使用 volatile MMIO 初始化地址 `0x10000000` 的 16550 UART，并在发送寄存器可用时写入字符。

#### Scenario: 输出启动文本

- **WHEN** hart 0 完成 UART 初始化并输出启动消息
- **THEN** 终端显示 `ECNU OSLab rCore kernel entered S-mode`

### Requirement: 格式化输出支持实验所需类型

内核 SHALL 通过 Rust `core::fmt` 输出有符号十进制、无前缀十六进制、带 `0x` 前缀的 64 位十六进制、字符和字符串。

#### Scenario: 检查所有格式

- **WHEN** hart 输出讲义规定的格式化验收消息
- **THEN** 终端显示 `d=-2025 p=2025 x=0x1234567880000000`、字符和字符串 `uart`

### Requirement: 并发输出保持消息完整

内核 MUST 将一整次 Rust 格式化输出作为互斥临界区。获取锁前 MUST 可嵌套地关闭当前 hart 的 S-mode 中断；释放锁后仅在最外层临界区结束时恢复原中断状态。

#### Scenario: 双 hart 同时报告启动状态

- **WHEN** hart 0 和 hart 1 各调用一次 `println!`
- **THEN** 终端包含两条完整消息，先后顺序可变但行内字符不交错

#### Scenario: 嵌套关中断保留原状态

- **WHEN** 同一 hart 嵌套进入两层关中断区域后逐层退出
- **THEN** 内层退出不提前开启中断，最外层退出后恢复首次进入前的状态

### Requirement: UART 初始化在双 hart 之间正确发布

hart 0 SHALL 以 Release 顺序发布 UART 初始化完成状态，hart 1 SHALL 以 Acquire 顺序等待该状态。

#### Scenario: hart 1 等待初始化

- **WHEN** hart 1 先于 hart 0 到达输出阶段
- **THEN** hart 1 等待初始化完成后再访问 UART

### Requirement: 实验可以自动验收

实验工程 MUST 提供 `make check`，检查 ELF、关键指令和规定输出。

#### Scenario: 完整实现通过检查

- **WHEN** 学生在 `lab-1` 目录执行 `make check`
- **THEN** 检查脚本最终输出 `lab-1 check: PASS`
