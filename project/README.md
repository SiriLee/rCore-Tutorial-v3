# rCore 课程项目：Linux 系统调用兼容层

让 rCore 能直接运行**未经修改的** RISC-V Linux 静态程序。

- 提交件：[`docs/proposal.md`](docs/proposal.md)
- 内部技术文档：[`docs/feasibility.md`](docs/feasibility.md)（可行性验证的实测过程与结论，不随提案提交）
- 需求规格：`openspec/changes/add-linux-syscall-compat/`
- 基线：`rCore-Tutorial-v3`，分支 `oslab-2026-base`，commit `c91bd3752b53ff48555aef4e3c7b8d5ddc8ee6e1`

## 目录说明

| 路径 | 内容 |
| --- | --- |
| `docs/` | 文档：提案、可行性验证报告，以及后续的设计说明 |
| `programs/` | Linux 侧测试程序的源码（Rust，交叉编译为静态 musl 程序） |
| `scripts/` | 构建、注入镜像、启动模拟器的脚本 |
| `evidence/` | 运行日志与对比数据，作为验收证据 |
| `third_party/` | 外部程序（如 busybox）的获取说明，**二进制不入库** |
| `build/` | 编译产物，由脚本生成，**不入库** |

内核改动**不在**本目录：程序装载、系统调用入口与兼容层等改动直接位于 `os/`、`easy-fs/` 等上游目录，由 git 在项目分支上跟踪。

## 快速开始

```bash
rustup target add riscv64gc-unknown-linux-musl   # 一次性

project/scripts/build-programs.sh    # 编译 Linux 侧测试程序
project/scripts/pack-fs-img.sh       # 构建内核并把测试程序注入文件系统镜像
project/scripts/run-qemu.sh          # 启动 QEMU（退出：Ctrl-A 后按 X）
```

进入 rCore 命令行后执行 `min_raw` 或 `min_std` 即可。

## 当前状态

- 可行性验证**已完成**：最小 Linux 程序能被装入并跳转执行，目前停在非法指令；三个拦路问题均已定位（见 `docs/feasibility.md`）。
- 下一步：按 `openspec/changes/add-linux-syscall-compat/tasks.md` 第 1 组推进。
