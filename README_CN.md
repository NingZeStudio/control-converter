# control-converter

在 [Zalith Launcher 2](https://github.com/ZalithLauncher/ZalithLauncher2) 和 [FoldCraftLauncher](https://github.com/FCL-Team/FoldCraftLauncher) 之间转换控件布局 JSON。

[![License: MIT](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![Python](https://img.shields.io/badge/python-3.8%2B-blue)](https://www.python.org/)
[![Demo](https://img.shields.io/badge/demo-cc.miawa.cn-9cf)](https://cc.miawa.cn)
[![API](https://img.shields.io/badge/api-api.cc.miawa.cn-orange)](https://api.cc.miawa.cn)

## 快速开始

```bash
# ZL -> FCL
python cc.py zl2fcl zl_layout.json fcl_layout.json

# FCL -> ZL
python cc.py fcl2zl fcl_layout.json zl_layout.json

# 自动检测格式
python cc.py auto input.json output.json
```

零依赖，仅需 Python 3.8+ 标准库。

## 安装

```bash
git clone https://github.com/NingZeStudio/control-converter.git
cd control-converter
```

无需 `pip install`，直接运行 `python cc.py` 即可。

**在线体验**: [cc.miawa.cn](https://cc.miawa.cn) -- 网页版转换工具。API 地址 [api.cc.miawa.cn](https://api.cc.miawa.cn)。

## 它能做什么

Zalith Launcher 2 和 FoldCraftLauncher 使用完全不同 JSON 架构来描述屏幕控件布局 -- 键码体系、颜色格式、尺寸单位、事件模型都不一样。这个工具负责两者之间的双向翻译：

- **按键映射** -- GLFW 键名 <-> FCL 整数键码（100+ 按键、3 个鼠标按钮、滚轮事件）双向转换
- **颜色转换** -- ZL Compose Color（带符号 Long sRGB 打包格式）<-> FCL ARGB int，含 alpha 混合
- **尺寸转换** -- ZL dp / 百分比 / wrap_content <-> FCL ABSOLUTE / PERCENTAGE（千分比），含 wrap_content dp 估算
- **样式映射** -- 按钮样式双向转换（文字、边框、填充、圆角、按下态）
- **事件翻译** -- 按键按下、启动器事件（滚轮、输入法切换、菜单）、发送文本、图层可见性切换
- **图层保留** -- ZL 图层 -> FCL 视图组，含组级可见性和排序
- **方向控制** -- FCL 摇杆/方向键 <-> ZL 摇杆 + 按钮网格
- **往返元数据** -- 在输出中嵌入 `_control_converter` 元数据，以便恢复原始布局

无法 1:1 映射的会使用有文档记录的 fallback，替换详情打印到 stderr。

## 功能对比

| 功能 | CLI | API |
|------|-----|-----|
| ZL -> FCL | `zl2fcl` | `/convert` |
| FCL -> ZL | `fcl2zl` | `/convert` |
| 自动检测 | `auto` | `/convert` |
| 方向控制 | `--include-directions` | `includeDirections` |
| 无损模式 | `--lossless` | `lossless` |
| 绝对尺寸转百分比 | `--absolute-as-percentage` | `absoluteAsPercentage` |
| 去除元数据 | `--strip-meta` | `stripMeta` |
| 安全可用模式 | `--usable` | `usable` |
| JSON 注释支持 | -- | `/convert-file` |
| 健康检查 | -- | `GET /health` |

## CLI 用法

```bash
python cc.py <模式> <输入文件> <输出文件> [选项]
```

### 模式

| 模式 | 方向 |
|------|------|
| `zl2fcl` | Zalith Launcher 2 -> FoldCraftLauncher |
| `fcl2zl` | FoldCraftLauncher -> Zalith Launcher 2 |
| `auto` | 自动检测格式并转换 |
| `api` | 启动 HTTP API 服务器 |

### 选项

| 选项 | 作用 |
|------|------|
| `--include-directions` | 将 FCL 方向控制近似为 ZL 按钮网格 |
| `--lossless` | 替换不支持的控件而非丢弃；同时转换 FCL 方向控制 |
| `--absolute-as-percentage` | 将 FCL 绝对 dp 尺寸转为 ZL 百分比尺寸 |
| `--strict` | 遇到不支持的字段时报错而非警告 |
| `--compact` | 输出紧凑 JSON（无空白） |
| `--strip-meta` | 从输出中移除 `_control_converter` 元数据 |
| `--usable` | ZL->FCL 结构安全模式，仅移除大面积空白遮挡控件 |
| `--aspect 16/9` | 方向控制近似时使用的屏幕宽高比（默认 1.778） |

## API 服务器

启动服务器：

```bash
python cc.py api --host 0.0.0.0 --port 8000
```

在线 API: [api.cc.miawa.cn](https://api.cc.miawa.cn)

### 端点

**POST /convert** -- 转换请求体中的布局。

```json
{
  "mode": "zl2fcl",
  "data": { ... },
  "strict": false,
  "stripMeta": false
}
```

响应：
```json
{ "ok": true, "data": { ... } }
```

**POST /convert-file** -- 转换上传的 JSON 文件。支持 `//` 和 `/* */` 注释。

```
POST /convert-file?mode=zl2fcl&strict=false
Content-Type: application/json

{ ... 含注释的布局 JSON ... }
```

**GET /health** -- 健康检查。

```json
{ "ok": true }
```

## 已知限制

- ZL `wrap_content` 尺寸无 FCL 等价物，根据文本长度和字号估算
- ZL `hideWhenMouse` / `hideWhenGamepad` 图层标志无 FCL 等价物，跳过并警告
- FCL 侧键 4-8 映射到滚轮事件作为 fallback
- FCL `KEY_KPCOMMA` 无精确 GLFW 键，fallback 到小键盘小数点

所有替换均有追踪记录，附带原因说明，输出到 stderr。

## 许可证

MIT -- 详见 [LICENSE](LICENSE)。
