# Binary Build Guide (Android JNI)

`cc.py` 的转换逻辑现以 **Rust** 为主实现，编译为 Android aarch64 cdylib `dist/libcc.so`（约 0.86MB，较 Go 版约 -79%；以实际构建为准）。
Go 版（`go/`，约 4.2MB）保留作回归参照，构建方式见文末。

## 架构（Rust 主线）

```
Java (LayoutConverter.java)
  └─ System.loadLibrary("cc")
     └─ JNI: Java_com_tungsten_fcl_util_LayoutConverter_convertFclToZl2Native
        └─ Rust: cc-rs/src/jni.rs → fcl_to_zl::convert_fcl_to_zl()
```

- **入口**：`cc-rs/src/jni.rs`（jni crate 0.21）
- **转换逻辑**：`cc-rs/src/`（自 `cc.py`/`go/` 完整移植，输出字节级一致）
- 依赖：serde_json（preserve_order + arbitrary_precision）、ryu、getrandom、jni

## 自行编译（Rust）

### 方式一：Android 本机编译（Termux）

Termux 上 host==target（aarch64-android），无需 NDK：

```bash
pkg install rust

cd control-converter/cc-rs
cargo build --release
# 产物: target/release/libcc.so
cp target/release/libcc.so ../dist/libcc.so
```

Termux 缺 build-tools 时，还需要把 aapt2 覆盖写到**用户级**配置
`~/.gradle/gradle.properties`（勿提交进仓库，否则 PC/CI 构建会坏）：

```
android.aapt2FromMavenOverride=/data/data/com.termux/files/usr/bin/aapt2
```

### 方式二：NDK 交叉编译（PC）

```bash
# aarch64-linux-android21 目标 + cargo-ndk（或手动配 linker）
rustup target add aarch64-linux-android
cargo install cargo-ndk
cd control-converter/cc-rs
cargo ndk -t arm64-v8a -p 21 -- build --release
```

`[profile.release]` 已含 `opt-level="z" + lto + codegen-units=1 + strip`。

### Android 壳工程签名（android/）

release 签名凭据只来自环境变量（`KEYSTORE_PATH` / `KEYSTORE_PASSWORD` /
`KEY_ALIAS` / `KEY_PASSWORD`）或本地 `android/release.keystore`（已被
.gitignore 排除）。凭据不齐时不挂接签名，`assembleRelease` 产出 unsigned
APK；仓库内不含任何密钥口令。

### CLI 测试工具

```bash
cargo build --release --example convert
CC_DETERMINISTIC=1 target/release/examples/convert <input.json> <output.json>
```

（`CC_DETERMINISTIC=1` 使生成的随机 ID 确定化，便于与 Go 版 `cmp` 字节对拍；
正常部署/使用时无需设置。）

### 回归验证

```bash
# 金样 + 真实布局对拍（需 CC_DETERMINISTIC=1，两侧 ID 序一致）
cd go && go build -o /tmp/ccgo . && cd ..
CC_DETERMINISTIC=1 /tmp/ccgo go/testdata/test_fcl_layout.json /tmp/out_go.json
CC_DETERMINISTIC=1 cc-rs/target/release/examples/convert go/testdata/test_fcl_layout.json /tmp/out_rs.json
cmp /tmp/out_go.json /tmp/out_rs.json && echo IDENTICAL
```

真实布局压测：`/sdcard/fcl/control/*.json`（见 AGENTS.md）。

## Go 版（回归参照保留）

```bash
cd control-converter/go
CGO_ENABLED=1 GOOS=android GOARCH=arm64 \
  go build -buildmode=c-shared -o ../dist/libcc-go.so .
# 或本机 CLI: go build -o /tmp/ccgo .
```

注意：arm64 gc 会跨语句融合浮点（+1ulp 偏差），`go/geometry.go` 已用
`//go:noinline`（fclRectSize/fclRectOrigin）打断以保持与 Rust 严格 IEEE 一致。

## 打包到 FCL

将 `dist/libcc.so`（Rust 版）复制到 FCL 项目的 jniLibs 目录：

```bash
cp dist/libcc.so /path/to/FoldCraftLauncher/FCL/src/main/jniLibs/arm64-v8a/libcc.so
```

Java 通过 `System.loadLibrary("cc")` 加载，接口与 Go 版完全相同。

## Python 版本

原始 Python 实现 `cc.py` 仍保留，可用于：
- 命令行批量转换
- Web API 服务
- 回归测试参照

```bash
python cc.py fcl2zl input.json output.json --lossless
```
