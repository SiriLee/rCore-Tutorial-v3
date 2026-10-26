# Lab 3 Trap and Interrupt Specification

## Purpose

定义实验三在 Rust/rCore 裸机内核中的 S-mode trap 现场、M-mode 时钟转发、
PLIC/UART 外部中断及可重复验收行为。

## Requirements

### Requirement: 内核 trap 现场可无损往返

内核 MUST 以与 `KernelTrapContext` 一致的 256 字节布局保存 x0 至 x31，调用
Rust handler 后恢复所有可写通用寄存器，并通过 `sret` 回到被打断位置。

#### Scenario: 时钟中断打断内核代码

- **WHEN** 任意 hart 在 S-mode 执行时收到一次已委托的中断
- **THEN** handler 返回后 PC、栈和通用寄存器 MUST 保持陷入前语义

### Requirement: 机器时钟被周期转发给 S-mode

系统 SHALL 为两个 hart 分别设置 `mtimecmp`、`mscratch` 和 M-mode trap 入口。
每次机器时钟中断 MUST 预约下一事件、置位 `sip.SSIP`，并由 S-mode handler
清除 SSIP；全局 tick 只由 hart 0 更新。

#### Scenario: 连续产生三个 tick

- **WHEN** 内核打开 SSIE 并等待时钟事件
- **THEN** tick MUST 单调增长到至少 3，且输出 `lab-3 timer test: PASS`

### Requirement: UART 输入经 PLIC 完整处理

系统 SHALL 把 QEMU virt UART IRQ 10 的优先级设为 1，只在 hart 0 的 S-context
使能该 IRQ，并对每个非零 claim 执行一次同 context 的 complete。

#### Scenario: 输入验收字符

- **WHEN** QEMU 串口收到字符 `K`
- **THEN** UART handler MUST 读取并回显字符，随后输出 `lab-3 uart test: PASS byte=K`

#### Scenario: 输入退格和换行

- **WHEN** 串口收到普通字符、Backspace 以及回车
- **THEN** 普通字符 MUST 回显，Backspace MUST 擦除一个终端字符，回车 MUST 规范化为换行

### Requirement: 分页后设备地址可访问

内核页表 MUST 以 R/W、不可执行、无 U 权限恒等映射 UART 页以及实验所用的
PLIC priority、enable、threshold/claim 页。

#### Scenario: 开启 Sv39 后到达 UART 外部中断

- **WHEN** 两个 hart 已写入 Lab 2 的内核页表 token 且 UART 产生 IRQ 10
- **THEN** PLIC claim、UART MMIO 和 complete MUST 均可访问且不产生页故障

### Requirement: 实验可以一键验收

项目 MUST 提供静态指令检查和双 hart QEMU 集成检查。

#### Scenario: 从干净状态验收

- **WHEN** 实验者执行 `make clean && make check`
- **THEN** 检查 MUST 验证 trap/timer CSR 指令、上下文布局、连续 tick 和 UART 输入，并以 `lab-3 check: PASS` 结束
