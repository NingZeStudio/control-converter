# AGENTS.md

## 项目概要

control-converter：Zalith Launcher 2 (ZL2) 与 FoldCraftLauncher (FCL) 控件布局 JSON 互转工具。
- `cc.py` — Python 参考实现（CLI + API）
- `cc-rs/` — **Rust 主线**，编译为 Android aarch64 cdylib（`dist/libcc.so`，约 0.86MB，较 Go 版约 -79%；源码变更后需按 BUILD.md 重新构建）
- `go/` — Go 版保留作回归参照（go build 出 CLI；`go/testdata/` 有金样）
- Rust 迁移记录见 `RUST_MIGRATION.md`

## Rust 版状态（已完成）

- 入口：`cc-rs/src/jni.rs` 导出两个 JNI 函数（Java 侧零改动签名）：
  - `convertFclToZl2Native`（正向，与 Go 版对齐，输出转义 `<>&`）
  - `convertZl2ToFclNative`（反向，与 cc.py 对齐，输出不转义 HTML）
- CLI 测试工具：`cargo build --release --example convert` → `convert [fcl2zl|zl2fcl] <in> <out>`
- 回归验证：金样 + 5 个真实布局**双向**与 Go/Python 字节级一致（CC_DETERMINISTIC=1）
- 反向转换参照 `cc.py`（Go 版未实现 zl2fcl）；`cc.py` 已加 CC_DETERMINISTIC 钩子
- JSON 栈：serde_json（preserve_order + arbitrary_precision）+ ryu + jni crate
- 浮点格式化：`utils.rs` 三个格式化器（PyFloat / Go strconv 'g' / encoding/json float）已与 Go 12 万值对拍 0 diff
- Android 应用：`android/`（**壳子来自 [zhizhu0002/ControlLayoutConverter](https://github.com/zhizhu0002/ControlLayoutConverter)**，Compose+Miuix UI，FCL/ZL1/ZL2 三格式互转 + WebView JS 引擎兜底 + 在线转换回退）
  - JNI ABI 与本项目一致（`com.tungsten.fcl.util.LayoutConverter`，双方法），已替换其 jniLibs 为 Rust 版 libcc.so
  - 本机构建（Termux，已趟通）：`cd android && export JAVA_HOME=$PREFIX/lib/jvm/java-21-openjdk && ./gradlew assembleDebug --no-daemon`
  - 依赖：`~/android-sdk/`（platform-37 官方 zip + licenses 伪目录，无 build-tools）、Termux 缺 build-tools 时把 `aapt2FromMavenOverride=$PREFIX/bin/aapt2` 写在**用户级** `~/.gradle/gradle.properties`（勿入库，PC/CI 会坏）
  - gradlew 首行 shebang 已改成本机路径；产物 `android/app/build/outputs/apk/debug/app-debug.apk`
  - PC 构建用作者原 build.bat 流程或 Android Studio 打开 android/

## 字节级一致的关键坑（改 Go 或 Rust 前必读）

1. **gc 浮点融合**：Go 规范允许跨语句融合，arm64 gc 会把 `(screenW-width)*x/y` 用高精度中间值算出 +1ulp 偏差。
   已在 `go/geometry.go` 用 `//go:noinline`（fclRectSize/fclRectOrigin）打断。Rust 是严格 IEEE，勿改动对应公式。
2. **HTML 转义**：Go 的 `OrderedMap.MarshalJSON` 内部用 `json.Marshal` 逐值转义 `<>&`（即使 Encoder SetEscapeHTML(false)）；
   Python `json.dumps` 不转义。Rust 侧 `go_html_escape_json` 后处理仅用于正向（fcl2zl）输出。
3. **Go map 迭代随机序**：`inferEventsFromGroupNames` 的平局选择曾依赖 map 序导致 Go 版输出非确定。
   两侧均已改为插入序保序结构（Go `groupIDMap` / Rust `GroupIdsByName`）。
4. **正则 run 分割**：`[A-Za-z0-9]+|[CJK]+` 交替——alnum 与 CJK 必须分开成 run（"时间1" → "时间"+"1"），勿合并。
5. **对拍方法**：两侧都认 `CC_DETERMINISTIC=1` 环境变量（ID 用计数器生成且调用序一致），`cmp` 直接字节对比。
6. **键序语义**：Python `dict.update`/赋值顺序是输出键序的权威。`joystickStyles` 必须在 `editorVersion` **之前**（Go/Rust 曾顺序相反，靠正向三方对拍发现）。Python `dict.insert` 对已存在键原位覆盖、新键追加尾部——与 serde_json preserve_order 的 `Map::insert` 一致。
7. **显示约定**：`textAlignment` 输出 `Center`（三端统一，2026-08-31 起；旧金样是 Left 已更新）。正向输出的 HTML 转义（`\u003c` 等）是 Go 行为、Python 不转义——正向对拍以 Go=RS 为准，反向以 PY=RS 为准。
8. **对拍参数**：Python 正向必须加 `--lossless` 才与 JNI 默认参数（lossless=true）一致。
9. **真实 ZL2 布局样本**：`/sdcard/dta5a60d.zl(1).json`（FullsUI）、`/sdcard/控件/*.zip` 内 versions/*.json——无 meta 的第三方布局，反向对拍用；8 项 fuzz 病态输入（null layers/错类型/缺键）全部通过。
10. **toString(对象) 三端渲染不同**：Go `fmt %v` 把 OrderedMap 渲染成 `&{[k1 k2] map[k1:v1]}`（map 键按字节排序），Python 是 `str(dict)`，Rust 切勿落到 serde_json 序列化。正向需要 Go 渲染——统一走 `utils.rs::go_to_string`（originId 回退已用）；新增可能 stringify 对象/数组的正向调用点时必须用同一入口。
11. **fcl_id 不改写版本位**：cc.py `str(uuid.UUID(hex))` 保留原始 128 位；Go `fclID` 会 OR 入 version/variant 位但仅用于正向，从不与反向对拍。Rust `fcl_id()` 必须保持不改写（zl_to_fcl 的 8 处调用全依赖它与 Python 字节一致）。
12. **rocker 平局选择必须保序**：`zl_joystick_styles_to_fcl_direction_styles` 对既有 ROCKER 样式的匹配遍历须按 cc.py dict 插入序（同名后值覆盖原位），用 HashMap `.values()` 直选会引入进程级随机序。
13. **除 10 是银行家舍入不是整除**：cc.py `scale_position_to_fcl` 是 `clamp_int(x/10)`（float 除 + `int(round())`，4999→500）；Rust 若写成 `x/10` 整除会得 499。反向仅在 meta 缺失重算几何时才走到该路径，对拍必须覆盖无 meta 输入。

## 真实 FCL 布局测试数据（Android 本机）

**位置**：`/sdcard/fcl/control/`（即 `/storage/emulated/0/fcl/control/`，Termux 需已授权存储权限）

| 文件 | 大小 | viewGroups | 控件数(viewData) | buttonStyles | 说明 |
|---|---|---|---|---|---|
| `00000000.json` | 32KB | 1 | ~0 | 1 | FCL 官方默认布局（最小样例，结构参照） |
| `4bf3d919.json` | 2.2MB | 100 | 200 | 109 | 最大最复杂的实例（回归压测首选） |
| `726214ca.json` | 622KB | 17 | - | 6 | 实例布局 |
| `dta5a60d.json` | 563KB | 14 | - | 6 | 实例布局 |
| `fa095f58.json` | 423KB | 14 | - | 3 | 实例布局 |

**FCL schema 注意**（与直觉不同，勿搞错）：
- 控件按钮在 `.viewGroups[].viewData`（object，非数组），**不在**顶层 `controls`
- 样式在顶层 `buttonStyles` / `directionStyles`，**不在** `styles`
- `styles/` 子目录的 `button_styles.json`、`direction_styles.json` 是独立样式文件
- `input/input_text.json` 是字符串数组（游戏输入配置，如 `["/gamemode crater"]`），**不是**控件布局，不能用作转换输入

## 测试环境备忘

- 本机是 Android/Termux（aarch64），**无 python/python3/node**，JSON 处理用 `jq`；rust/cargo 已装（pkg install rust）
- Go 可用（`pkg install golang`）；Termux 自带 aarch64-linux-android-clang 与 jni.h（`$PREFIX/include/jni.h`）
- Termux 上 cargo 直接 `--release` 构建即为 Android aarch64 cdylib（host==target），无需 NDK
- `go/testdata/` 金样：`output_go.json`（52309B，与现行 lossless=true Go 输出一致）；`output_python.json` 为旧参数遗留，仅作历史参照，回归以上节对拍方法为准
