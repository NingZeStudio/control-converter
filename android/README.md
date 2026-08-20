# Control-Converter JNI 测试应用

一个最小的 Android 应用，用于在真机上验证 `dist/libcc.so`（Go 实现的
FCL→ZL2 布局转换 JNI 库）的调用链路。

## JNI 接口

| 项 | 值 |
|---|---|
| 库名 | `cc`（`System.loadLibrary("cc")`） |
| 宿主类 | `com.tungsten.fcl.util.LayoutConverter` |
| 方法 | `static native String convertFclToZl2Native(String inputPath, String outputPath)` |
| 返回 | `null` = 成功；否则为错误消息字符串 |

JNI 符号（由 Go `go/main.go` 的 `//export` 固定生成）：

```
Java_com_tungsten_fcl_util_LayoutConverter_convertFclToZl2Native
```

## 应用功能

- 启动时把打包在 assets 里的 `test_fcl_layout.json`（Go 测试样例）复制到应用私有目录
- 输入/输出路径可编辑（默认填好）
- 点击「转换」→ 后台线程调用 JNI → 显示耗时、输出文件信息、输出 JSON 预览
- 转换失败时显示错误消息（例如文件路径错误、JSON 解析失败）

## 构建

### 方式 A：Android Studio（推荐）

1. 用 Android Studio 打开本目录（`android/`）
2. 等待 Gradle 同步（AGP 9.2.0 / Gradle 9.4.1，首次会自动下载依赖）
3. 连接 arm64 真机（开启 USB 调试），点 Run

> 注意：`libcc.so` 只编译了 **arm64-v8a** 架构，应用也仅打包该 ABI，
> 请使用 arm64 真机（绝大多数 2017 年后的手机）。x86 模拟器无法加载该库。

### 方式 B：命令行

环境要求（本机已具备，路径见脚本）：

- Android Studio 自带 JBR（JDK 25）：`D:\app\as\jbr`
- Android SDK：`C:\Users\HASEE_Z7\AppData\Local\Android\Sdk`
- Gradle 9.4.1（wrapper 缓存发行版）

```bat
build.bat
```

产物：`app\build\outputs\apk\debug\app-debug.apk`

安装：

```bat
adb install -r app\build\outputs\apk\debug\app-debug.apk
```

## 重新编译 JNI 库（可选）

`dist/libcc.so` 是预编译产物。如需用最新 Go 代码重新编译：

```bat
rebuild_so.bat
```

脚本会：

1. 用 NDK r25 的 `aarch64-linux-android21-clang` 交叉编译
   `CGO_ENABLED=1 GOOS=android GOARCH=arm64 go build -buildmode=c-shared`
2. 刷新 `dist/libcc.so` 和 `app/src/main/jniLibs/arm64-v8a/libcc.so`

手动命令（等价于脚本）：

```bash
cd ../go
export NDK_ROOT=<你的 NDK 路径>
export CC=$NDK_ROOT/toolchains/llvm/prebuilt/windows-x86_64/bin/aarch64-linux-android21-clang.cmd
CGO_ENABLED=1 GOOS=android GOARCH=arm64 go build -buildmode=c-shared -o ../dist/libcc.so .
```

## 项目结构

```
android/
├── build.bat                     # 一键命令行构建 APK
├── rebuild_so.bat                # 用 NDK 重编译 libcc.so
├── settings.gradle.kts
├── build.gradle.kts
├── app/
│   ├── build.gradle.kts          # AGP 9.2.0, compileSdk 37, minSdk 21, ABI arm64-v8a
│   └── src/main/
│       ├── AndroidManifest.xml
│       ├── assets/test_fcl_layout.json   # 测试样例（Go testdata 复制）
│       ├── jniLibs/arm64-v8a/libcc.so    # JNI 库（dist 复制）
│       ├── java/com/tungsten/fcl/util/LayoutConverter.java  # JNI 桥
│       ├── java/com/tungsten/fcl/jnitest/MainActivity.java  # 测试 UI
│       └── res/values/strings.xml
```
