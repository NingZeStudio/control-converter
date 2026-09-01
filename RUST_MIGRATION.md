# Rust 迁移方案（已完成）

> **状态：迁移已完成并通过字节级回归验证。** Rust 主线位于 `cc-rs/`，
> Go 版保留作回归参照。本记录保留原始方案 + 实施中发现的额外坑（见文末"实施补充"）。

> 目标：将 `go/` 目录（约 3800 行）的 FCL→ZL2 转换逻辑移植到 Rust，
> 编译为 Android aarch64 cdylib（`libcc.so`），JNI 接口与转换输出和 Go 版 100% 一致。
> 迁移期间保留 `go/` 目录作为回归验证参照。

## 实施结果

| 指标 | Go 版 | Rust 版（实测） |
|---|---|---|
| libcc.so 体积 | 4,221,184 B | 约 0.86MB（-79%，随功能演进以实际构建为准） |
| JNI 接口 | 不变 | 不变（Java 侧零改动） |
| 转换输出 | — | 金样 + 5 个真实布局字节级一致 |
| 依赖 | Go runtime | serde_json / ryu / getrandom / jni |

## 实施补充（比原方案多发现的坑）

1. **gc 浮点融合（最隐蔽）**：arm64 gc 会把 `screenW - Max(1, screenW*w/1000)`
   融合为高精度中间值计算，产生 +1ulp 偏差并连锁影响按钮排序/候选阈值。
   复现实验见 RUST_MIGRATION 历史；解法是在 Go 侧 `fclRectSize`/`fclRectOrigin`
   加 `//go:noinline` 强制严格 IEEE（与 Rust 一致，且跨平台更稳）。
   注意 `math.Float64frombits(Float64bits(x))` 往返会被 gc 消除，无法阻止融合。
2. **HTML 转义**：Go `OrderedMap.MarshalJSON` 内部 `json.Marshal` 逐值转义 `<>&`，
   外层 `SetEscapeHTML(false)` 不生效。Rust 在 `jsonio.rs::go_html_escape_json`
   对输出做字符串状态扫描后处理。
3. **Go map 迭代随机**：`inferEventsFromGroupNames` 平局选择依赖 map 序，
   Go 版输出本身非确定（多次运行互不相同）。两侧均改插入序保序结构后修复。
4. **正则 run 分割**：`[A-Za-z0-9]+|[CJK]+` 交替——"时间1" 必须拆成 "时间"+"1"
   两个 run；Rust 初版把两类合成一个 run 导致 words 集合不同。
5. **serde_json 特性组合**：`preserve_order`（IndexMap 保键序）+
   `arbitrary_precision`（Number 保留原始文本，对应 Go json.Number）缺一不可。
6. **最短浮点 digits**：Rust 内建 Grisu 与 Go Ryu 在平局 tie 上选数字可能不同
   （…592 vs …593）；数字部分统一用 `ryu` crate，格式规则再按 Go 移植。
7. **对拍基础设施**：`CC_DETERMINISTIC=1` 使两侧 ID 由计数器生成（调用序一致
   则 ID 一致）；Go 侧加了测试 CLI（`go run . <in> <out>`）。


## 一、现状分析

```
Java LayoutConverter.convertFclToZl2Native(inputPath, outputPath)
  └─ JNI 符号: Java_com_tungsten_fcl_util_LayoutConverter_convertFclToZl2Native
     └─ Go convertFCLToZL(source,
          includeDirections = false,
          strict            = false,
          aspect            = 16/9,
          lossless          = true,
          absoluteAsPercentage = false)   ← 参数全部硬编码 (go/main.go:131)
```

**关键结论：JNI 层极薄** —— 只有一个导出函数，输入输出都是**文件路径**（不是 JSON 字符串），
返回 `NULL` 表示成功、错误消息 jstring 表示失败。这大幅降低了 JNI 层的迁移风险。

产物对比基线：Go 版 `dist/libcc.so` = 4.2 MB；Rust 版预期 **300~600 KB**。

## 二、模块对应表

| Go 文件 | 行数 | Rust 模块 | 移植说明 |
|---|---|---|---|
| `main.go` | 143 | `src/jni.rs` | jni crate 重写；`parseAspect` 等辅助函数随迁 |
| `orderedmap.go` | 181 | **删除** | 用 serde_json `preserve_order` + `arbitrary_precision` 特性替代 |
| `jsonio.go` | 207 | `src/jsonio.rs` | `stripJSONComments` 需手工移植；缩进/换行行为需对齐 |
| `constants.go` | ~340 | `src/constants.rs` | 纯数据表，机械搬运 |
| `utils.go` | 920 | `src/utils.rs` | **风险最高**：pyRound、PyFloat、颜色位运算 |
| `styles.go` | 353 | `src/styles.rs` | OrderedMap 构造 → `json!` 宏 |
| `events.go` | ~250 | `src/events.rs` | `strconvQuote`（Go 引号转义）需移植 |
| `geometry.go` | ~500 | `src/geometry.rs` | 浮点几何计算，逻辑直译 |
| `direction.go` | ~330 | `src/direction.rs` | 逻辑直译 |
| `buttons.go` | ~230 | `src/buttons.rs` | 逻辑直译 |
| `fcl_to_zl.go` | ~390 | `src/fcl_to_zl.rs` | 主流程 |
| （全局状态） | — | `src/context.rs` | `warnedMessages`/`substitutionCounts` → `ConversionContext` 结构体 |

## 三、关键技术决策

### 3.1 JSON 层（替代 OrderedMap 的核心决策）

```toml
serde_json = { version = "1", features = ["preserve_order", "arbitrary_precision"] }
```

- `preserve_order` → 内部改用 IndexMap，**键序与插入序一致**（对应 Python dict / Go OrderedMap）
- `arbitrary_precision` → 数字保留**原始文本**（对应 Go `json.Number` / `UseNumber()`）。
  输入 `"1.50"` 输出仍是 `"1.50"` 而不是被归一化为 `1.5`。没有这个特性会产生字节级 diff。

### 3.2 PyFloat 浮点格式化（最高风险点）

`go/utils.go:27` 自定义了 Python 风格浮点序列化：

```go
// 整数值且 |v| < 1e16 → "50.0"（strconv.FormatFloat 'f', 1）
// 其他            → 'g' 最短表示（strconv.FormatFloat 'g', -1）
```

Go 的 `'g'` 格式与 Rust serde_json 默认的 ryu 输出**不同**（如 `1e+06` vs `1e6`）。
**必须手写格式化函数**逐字对齐 Go 的 `'g'` 行为（指数阈值、`e+06` 补零格式），
对计算产生的 f64（几何/缩放结果）统一走 `PyFloat::to_json()`。
验证方式：随机生成 10 万个 f64，对比 Go 与 Rust 格式化输出，必须 0 diff。

### 3.3 pyRound 银行家舍入

`go/utils.go:174` 实现了 Python 风格 round-half-to-even。
Rust 的 `f64::round` 是四舍五入（half away from zero），**不能直接用**，
照抄 Go 的实现即可（Trunc + Mod 判断）。

### 3.4 全局状态

Go 用包级全局变量（`warnedMessages`、`substitutionCounts`），每次 JNI 调用重置。
Rust 中改为 `ConversionContext` 结构体，从入口一路传 `&mut`——
顺便消除全局可变状态，也天然支持未来多线程调用。

### 3.5 随机 ID

`shortID()`/`fclID()`（`go/utils.go:734`）用 crypto/rand 生成 12 位 hex / UUID v4。
Rust 用 `getrandom` crate。**输出含随机 ID，字节级 diff 测试需处理**（见第五节）。

### 3.6 JNI 层（jni crate 0.21）

```rust
#[no_mangle]
pub extern "system" fn Java_com_tungsten_fcl_util_LayoutConverter_convertFclToZl2Native(
    env: *mut jni::sys::JNIEnv, _class: jni::objects::JClass,
    input: jni::sys::jstring, output: jni::sys::jstring,
) -> jni::sys::jstring {
    // JNIEnv::from_raw + get_string + fs 操作 + 返回 null / JString
}
```

- 语义照抄 Go 版：成功返回 NULL，失败返回错误消息字符串（Java 侧 `LayoutConverter.java` 零改动）
- stderr 警告：`eprintln!` 即可，Android 上自动进 logcat（与 Go 行为一致）
- panic 安全：入口包 `catch_unwind` + `std::panic::catch_unwind` 前置 `panic::set_hook`（静默），
  防止跨 FFI 边界 unwind 造成 UB

## 四、分阶段计划

### Phase 0：工程脚手架（0.5 天）
- `cargo init --lib cc-rs`，`crate-type = ["cdylib"]`，release profile 体积优化：
  ```toml
  [profile.release]
  opt-level = "z"
  lto = true
  codegen-units = 1
  strip = true
  panic = "abort"   # 注意：与 catch_unwind 冲突，若用 catch_unwind 则去掉此项
  ```
- `.cargo/config.toml` 配置 Android target linker（NDK 交叉编译用）
- Termux 本机编译：直接 `cargo build --release`（Termux 自带 aarch64 clang）

### Phase 1：基础层（1 天）
`constants.rs` + `context.rs` + `utils.rs`（纯函数部分：类型转换、pyRound、颜色转换）
+ `jsonio.rs` + PyFloat 格式化器（含 10 万随机值对拍测试）

### Phase 2：转换核心（2~3 天）
`geometry.rs` → `events.rs` → `styles.rs` → `direction.rs` → `buttons.rs` → `fcl_to_zl.rs`
按依赖顺序移植，每个模块配单测（用 `testdata/test_fcl_layout.json` 的片段做输入）。

### Phase 3：JNI 入口 + 集成（0.5 天）
`jni.rs` + 金样回归通过 + Android 测试 App（`android/` 目录已有 jnitest 工程）实机验证。

### Phase 4：收尾（0.5 天）
- 体积测量与优化（必要时 `cargo-zigbuild` / upx）
- 更新 `BUILD.md`/`README.md`（go/ 与 Rust 双轨说明）
- **不删除 `go/`**：作为参照保留，README 注明 Rust 版为后续主线

## 五、验证策略（回归测试）

金样：`go/testdata/output_go.json`（与现行 lossless=true Go 输出一致）；`output_python.json` 为旧参数遗留历史参照。

```bash
# 1. Rust 版转换测试输入
./target/aarch64-linux-android/release/libcc.so → 交叉测试用宿主端 cdylib 即可（转换逻辑与平台无关）

# 2. diff 对比，排除随机 ID 字段
jq 'walk(if type == "object" then with_entries(
      select(.key | test("^(id|uuid|_id)$") | not)) else . end)' \
     actual.json > actual_stripped.json
diff <(jq -S . actual_stripped.json) <(jq -S . go/testdata/output_go.json)
```

补充验证：
1. **PyFloat 对拍**：Go 写个小程序批量格式化随机 f64，Rust 单测对齐（Phase 1 完成）
2. **Go 版双跑**：同一输入分别跑 Go so 与 Rust so，除随机 ID 外逐字节 diff
3. **Android 实机**：用 `android/app/src/main/assets/test_fcl_layout.json` 走 jnitest App
4. **追加金样**：若现有单一金样覆盖不足，用 `cc.py` 批量生成多组（含 --lossless、directions 等开关组合）

## 六、风险清单

| 风险 | 影响 | 对策 |
|---|---|---|
| 浮点格式化不一致 | 输出字节 diff | 3.2 节专用格式化器 + 10 万值对拍 |
| 数字精度归一化 | `1.50`→`1.5` | `arbitrary_precision` 特性 |
| 舍入方向差异 | 尺寸/坐标偏 1px | 照抄 pyRound，禁用 `f64::round` |
| Go json 转义差异 | compact 模式 `<>` 转义不同 | JNI 路径只用缩进模式（EscapeHTML=false），对齐无碍；CLI compact 模式需注意 |
| panic 跨 FFI | UB/崩溃 | `catch_unwind` 包裹入口（放弃 `panic="abort"`） |
| 金样只有一组 | 覆盖不足 | 用 cc.py 生成多场景金样 |
| 依赖体积膨胀 | so 变大 | 只用 serde_json/getrandom/jni 三个依赖，测量后按需优化 |

## 七、预期收益

| 指标 | Go 版 | Rust 版（预期） |
|---|---|---|
| libcc.so 体积 | 4.2 MB | 300~600 KB（-85%+） |
| JNI 接口 | 不变 | 不变（Java 侧零改动） |
| 转换输出 | 100% 一致 | 100% 一致（金样验证） |
| 内存占用 | Go runtime 堆 + GC | 无 runtime，按需分配 |
