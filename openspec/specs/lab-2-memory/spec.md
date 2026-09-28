# Lab 2 Memory Management Specification

## Purpose

本规格定义 Lab 2 裸机内核的地址类型、物理页帧生命周期、Sv39 页表映射、内核地址空间权限、分页启用和双 hart 可见性，并给出可以由内核自检、反汇编和 QEMU 串口输出观察的验收条件。

## Requirements

### Requirement: 地址类型正确分解页号与偏移

内核 SHALL 使用独立类型表示物理地址、虚拟地址、物理页号和虚拟页号，并按 4 KiB 页计算 floor、ceil、页内偏移和 Sv39 三级索引。

#### Scenario: 检查跨页地址

- **WHEN** 测试地址 `0x12345`
- **THEN** floor 为 `0x12`、ceil 为 `0x13`、页内偏移为 `0x345`

### Requirement: 页帧可以分配并自动回收

页帧分配器 MUST 只管理内核镜像末尾至 `0x88000000` 的完整物理页，分配时清零页面，并在 `FrameTracker` 释放时归还页面。

#### Scenario: 分配释放闭环

- **WHEN** 分配一页、写入数据并让 tracker 离开作用域后再次分配
- **THEN** 页面可以重新分配且内容已清零

#### Scenario: 物理页耗尽

- **WHEN** 没有空闲页帧
- **THEN** 分配返回 `None`，不覆盖仍在使用的页面

### Requirement: Sv39 页表支持映射查询和取消映射

页表 MUST 建立三级 Sv39 映射，拒绝重复映射和非法权限；只读查询不得分配新的页表页。

#### Scenario: 映射访问撤销

- **WHEN** 将一个 VPN 映射到 PPN 后查询并取消映射
- **THEN** 查询先返回相同 PPN，取消后返回未映射

#### Scenario: 拒绝重复映射

- **WHEN** 对已映射 VPN 再次调用 map
- **THEN** 返回错误且原 PTE 保持不变

### Requirement: 内核映射满足分段权限

内核 SHALL 对代码段使用 `R-X`，只读数据使用 `R--`，数据、BSS、栈和剩余 RAM 使用 `RW-`，UART MMIO 使用 `RW-`，且这些映射不设置 `U` 位。

#### Scenario: 检查 W^X

- **WHEN** 查询内核代码页和可写数据页
- **THEN** 普通页面均不同时具有 W 与 X 权限

### Requirement: 双 hart 启动栈保持独立

内核 SHALL 为 hart 0 和 hart 1 分别保留至少 16 KiB、互不重叠且包含在 BSS 内的启动栈，以容纳 Lab 2 页表构造和自检调用链。

#### Scenario: 内存自检后 hart ID 仍然正确

- **WHEN** 两个 hart 完成内存自检并启用分页
- **THEN** 两个 hart 的栈内容互不覆盖，串口分别输出 hart 0 和 hart 1 的分页启用消息

### Requirement: 两个 hart 使用同一内核页表

hart 0 SHALL 完成页表构建后以 Release 发布 satp token；每个 hart SHALL 以 Acquire 读取 token，写入 `satp` 并执行 `sfence.vma`。

#### Scenario: 双 hart 启用分页

- **WHEN** QEMU 使用 `-smp 2` 启动内核
- **THEN** 两个 hart 均报告 paging enabled，且串口输出保持可用

### Requirement: 实验可以自动验收

实验工程 MUST 提供 `make check`，检查 `satp`、`sfence.vma` 和规定的内核自检输出。

#### Scenario: 完整实现通过检查

- **WHEN** 学生在 `lab-2` 目录执行 `make check`
- **THEN** 检查脚本最终输出 `lab-2 check: PASS`
