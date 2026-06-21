# Binary Build Guide (Android JNI)

`cc.py` 的转换逻辑已用 [Go](https://go.dev/) 重写，并通过 `c-shared` 构建模式编译为 Android aarch64 JNI 共享库 `dist/libcc.so`（约 4 MB）。

## 架构

```
Java (LayoutConverter.java)
  └─ System.loadLibrary("cc")
     └─ JNI: Java_com_tungsten_fcl_util_LayoutConverter_convertFclToZl2Native
        └─ Go: convertFCLToZL()  (fcl_to_zl.go)
```

- **入口**：`go/main.go` 中的 `//export` JNI 函数
- **转换逻辑**：`go/fcl_to_zl.go`（从 `cc.py` 完整移植）
- **输出**：与 Python 版 100% 一致

## 自行编译

### 依赖

- [Go 1.21+](https://go.dev/dl/)
- [Android NDK r25+](https://developer.android.com/ndk/downloads)

### 编译命令

```bash
cd control-converter/go

# 设置 NDK 编译器路径（按实际 NDK 版本调整）
export NDK_ROOT=/path/to/android-ndk
export CC=$NDK_ROOT/toolchains/llvm/prebuilt/<host>/bin/aarch64-linux-android21-clang

# 交叉编译 c-shared 库
CGO_ENABLED=1 GOOS=android GOARCH=arm64 \
  go build -buildmode=c-shared -o ../dist/libcc.so .
```

编译产物：
- `dist/libcc.so` — Android aarch64 JNI 共享库
- `dist/libcc.h` — C 头文件（JNI 函数声明，仅供参考）

### 打包到 FCL

将 `dist/libcc.so` 复制到 FCL 项目的 jniLibs 目录：

```bash
cp dist/libcc.so /path/to/FoldCraftLauncher/FCL/src/main/jniLibs/arm64-v8a/libcc.so
```

Android 系统安装 APK 时会自动释放到 `nativeLibraryDir` 并赋予执行权限，Java 通过 `System.loadLibrary("cc")` 加载。

## Python 版本

原始 Python 实现 `cc.py` 仍保留，可用于：
- 命令行批量转换
- Web API 服务
- 回归测试参照

```bash
python cc.py fcl2zl input.json output.json --lossless
```
