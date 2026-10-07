# 外部程序

本目录只保存**获取说明**，二进制不入库：体积较大，且随时可用脚本重新取得。

## busybox（riscv64 静态版本）

优先使用 **musl 版本**：其启动要求低于 glibc 版本，需要的系统调用更少。

### 方式一：Debian（glibc 静态）

```bash
cd project/third_party
curl -LO https://deb.debian.org/debian/pool/main/b/busybox/busybox-static_1.38.0-3+b1_riscv64.deb
dpkg-deb -x busybox-static_*.deb unpacked/
mkdir -p bin && cp unpacked/bin/busybox bin/
file bin/busybox          # 应显示 statically linked
```

### 方式二：Alpine（musl）

从 <https://dl-cdn.alpinelinux.org/alpine/latest-stable/main/riscv64/> 取 `busybox-*.apk`，
解包后用 `file` 确认所取二进制为 `statically linked`（Alpine 的默认 busybox 是动态链接的，
需要确认后再放入 `bin/`）。

## 使用

取得 `bin/busybox` 后，`scripts/pack-fs-img.sh` 会自动把它一并注入文件系统镜像，
之后即可在 rCore 命令行执行 `busybox echo`、`busybox cat`、`busybox ls /` 等命令。
