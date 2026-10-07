## Purpose

定义内核按 Linux 约定装载可执行文件、构造进程启动参数并处理系统调用的可观察行为，使未经修改的 RISC-V Linux 静态程序能够在 rCore 上运行。

## ADDED Requirements

### Requirement: 内核可以装载段非页对齐的 Linux 可执行文件

内核 SHALL 按可执行文件中记录的段偏移信息把各段放入内存，MUST NOT 假设段的起始地址与页边界对齐。

#### Scenario: 执行段的页内偏移非零

- **WHEN** 运行一个执行段的起始地址不在页边界上的 Linux 程序
- **THEN** 该程序的入口地址处存在正确的指令，程序开始执行且不触发非法指令

#### Scenario: 数据段大于文件中的长度

- **WHEN** 装载一个数据段的长度大于其在文件中实际内容长度的程序
- **THEN** 超出部分被清零，程序读取到 0 而不是残留数据

### Requirement: 进程按 Linux 约定接收启动参数

内核 MUST 在进程入口处按 Linux 约定摆放启动参数，使程序能够从栈顶取得参数个数与参数列表，并能够取得环境变量与辅助向量。

#### Scenario: 程序读取命令行参数

- **WHEN** 以 `min_std hello world` 启动程序
- **THEN** 程序打印出的参数列表为 `["min_std", "hello", "world"]`

#### Scenario: 静态标准库完成启动

- **WHEN** 启动一个静态链接 musl 的程序
- **THEN** 程序完成标准库初始化并执行到 `main`

### Requirement: 系统调用按进程的调用约定分派

内核 SHALL 为每个进程记录其所使用的调用约定，并在系统调用入口按该约定分派；系统调用入口 SHALL 支持最多 6 个参数。

#### Scenario: 两套约定的程序先后运行

- **WHEN** 先后运行一个 rCore 既有用户程序与一个 Linux 程序
- **THEN** 两者均按其各自的调用约定得到处理，既有程序的行为保持不变

#### Scenario: 使用 6 个参数的调用

- **WHEN** 一个 Linux 程序发起使用第 4 至第 6 个参数的系统调用
- **THEN** 内核收到完整的 6 个参数，而不是只收到前 3 个

### Requirement: 未实现的系统调用不得导致内核崩溃

内核 MUST 在收到尚未实现的系统调用编号时向发起调用的进程返回 `-ENOSYS`，且 MUST NOT 因此终止内核。

#### Scenario: 调用尚未实现的编号

- **WHEN** 一个进程发起内核尚未实现的系统调用编号
- **THEN** 该进程收到 `-ENOSYS`，内核继续运行并可以继续调度其他进程

### Requirement: 标准库程序可以完成文件与内存操作

内核 SHALL 提供静态链接标准库程序启动与运行所需的文件类与内存类系统调用，包括打开文件、定位读写位置、查询文件状态、读取目录，以及堆与匿名内存映射。

#### Scenario: 写入后读回文件

- **WHEN** 程序写入一个文件后再读回该文件
- **THEN** 读回的内容与写入的内容一致，且与在 Linux 主机上执行同一操作的结果相同

#### Scenario: 查询文件状态

- **WHEN** 程序对一个已存在的文件查询状态
- **THEN** 返回的大小与类型与该文件的实际情况一致

### Requirement: 可以运行真实的 Linux 工具集

内核 SHALL 支持运行 busybox 的 `echo`、`cat` 与 `ls` 子命令，且其输出 SHALL 与在 Linux 主机上运行同一程序的结果逐行一致。

#### Scenario: 运行 busybox 子命令

- **WHEN** 在 rCore 命令行执行 `busybox echo`、`busybox cat`（绝对路径文件）与 `busybox ls /`
- **THEN** 三条命令的输出与在 Linux 主机上运行同一 busybox 的输出逐行一致
