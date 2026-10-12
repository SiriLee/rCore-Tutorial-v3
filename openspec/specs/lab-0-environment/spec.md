# Lab 0 Environment and rCore Build Specification

## Purpose

定义课程环境、固定 rCore 源码、RISC-V 构建产物、QEMU 首次启动和 OpenSpec 配置的可重复验收要求。

## Requirements

### Requirement: Rust 交叉编译环境完整

开发环境 SHALL 提供 rustup 管理的 Rust/Cargo、`riscv64gc-unknown-none-elf` target、rust-src、LLVM tools、rustfmt、clippy 和 cargo-binutils。

#### Scenario: 检查交叉编译工具

- **WHEN** 学生从新终端执行 Lab 0 环境检查命令
- **THEN** Rust、Cargo、RISC-V target、`rust-objcopy`、`rust-objdump` 和 `rust-nm` MUST 全部可执行

### Requirement: 使用固定的 rCore 基线

课程仓库 MUST 位于 `oslab-2026-base` 分支，并 MUST 指向 commit `c91bd3752b53ff48555aef4e3c7b8d5ddc8ee6e1`。

#### Scenario: 核对源码版本

- **WHEN** 学生执行 `git branch --show-current` 和 `git rev-parse HEAD`
- **THEN** 分支名与完整 commit MUST 分别等于课程规定值

### Requirement: 构建产生 RISC-V 内核与文件系统镜像

项目 SHALL 使用 Cargo/LLVM 构建 RV64 内核 ELF 和裸二进制，并生成用户文件系统镜像。

#### Scenario: 执行 make build

- **WHEN** 学生在 `os/` 执行 `make build`
- **THEN** 内核 ELF、`os.bin` 和 `fs.img` MUST 存在，ELF MUST 为 RISC-V 64 位且包含 `_start` 与 `rust_main`

### Requirement: rCore 可以在 QEMU 中启动

系统 SHALL 使用 RustSBI 和 QEMU virt 装载内核，并进入用户 shell。

#### Scenario: 运行 hello_world

- **WHEN** 学生执行 `make run` 并在 `Rust user shell` 输入 `hello_world`
- **THEN** 终端 MUST 输出 `Hello world from user mode program!`

### Requirement: OpenSpec 配置可严格校验

仓库根目录 MUST 包含使用简体中文上下文的 `openspec/config.yaml`，实验规格 MUST 通过 strict 校验。

#### Scenario: 校验全部规格

- **WHEN** 学生执行 `openspec validate --all --strict --no-interactive`
- **THEN** Lab 0 规格 MUST 校验成功且命令 MUST 返回零状态

### Requirement: 环境和静态产物可以自动复核

材料 SHALL 提供不会安装软件或修改源码的检查脚本，用于复核 Ubuntu、工具链、固定源码版本、构建产物和 OpenSpec 状态。

#### Scenario: 执行 Lab 0 检查脚本

- **WHEN** 学生完成构建和规格安装后执行 `scripts/check-env.sh` 并传入 rCore 仓库路径
- **THEN** 所有静态条件满足时脚本 MUST 输出 `lab-0 environment check: PASS`
- **AND** 脚本 MUST NOT 把静态检查结果当作 `make run` 和 `hello_world` 的替代证据
