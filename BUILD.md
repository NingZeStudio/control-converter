# Binary Build Guide (Android / Termux)

`cc.py` 已通过 [Nuitka](https://nuitka.net/) 编译为 Android aarch64 原生二进制文件 `dist/cc`（约 24 MB，单文件、免依赖）。

## 快速使用

```bash
# 直接运行（无需 Python 环境）
./dist/cc --help

# ZL → FCL
./dist/cc zl2fcl input.json output.json

# FCL → ZL
./dist/cc fcl2zl input.json output.json --lossless

# 自动检测格式
./dist/cc auto input.json output.json

# 启动 API 服务器
./dist/cc api --host 0.0.0.0 --port 8000
```

## 自行动手编译

### 依赖安装（Termux）

```bash
pkg install python clang patchelf termux-elf-cleaner make -y
pip install nuitka
```

### 一键编译

```bash
cd control-converter

# 创建 ldd 替代（Nuitka 需要，但 Android 无原生 ldd）
cat > /data/data/com.termux/files/usr/bin/ldd << 'EOF'
#!/data/data/com.termux/files/usr/bin/python3
"""ldd replacement for Termux/Android."""
import subprocess, sys, os, re

LIB_PATHS = [
    '/data/data/com.termux/files/usr/lib',
    '/system/lib64',
    '/system/lib',
]

def find_lib(name):
    for p in LIB_PATHS:
        candidate = os.path.join(p, name)
        if os.path.exists(candidate):
            return candidate
    return None

for f in sys.argv[1:]:
    if not os.path.isfile(f):
        continue
    try:
        out = subprocess.check_output(['readelf', '-d', f], stderr=subprocess.DEVNULL).decode()
        needed = re.findall(r'Shared library:\s*\[(.+?)\]', out)
        for lib in needed:
            found = find_lib(lib)
            if found:
                print(f'\t{lib} => {found} (0x0000000000000000)')
            else:
                print(f'\t{lib} => not found')
    except Exception:
        pass
EOF
chmod +x /data/data/com.termux/files/usr/bin/ldd

# 编译
python3 -m nuitka --onefile --output-dir=dist --output-filename=cc cc.py
```

编译完成后，二进制文件位于 `dist/cc`，可直接分发到其他 Android 设备使用。

### 清理

```bash
rm -rf dist/cc.build dist/cc.dist dist/cc.onefile-build
```

## 二进制与 Python 脚本对比

| 特性 | `python cc.py` | `./dist/cc` |
|------|---------------|-------------|
| 需要 Python | 是 | 否 |
| 文件大小 | ~100 KB | ~24 MB |
| 启动速度 | 快 | 更快（预编译） |
| 分发便捷 | 需 Python 3.8+ | 单文件、免依赖 |
| 支持平台 | 全平台 | Android aarch64 |

## 完整 CLI 选项

```
用法: cc <模式> [输入] [输出] [选项]

模式:
  auto      自动检测格式并转换
  zl2fcl    Zalith Launcher 2 → FoldCraftLauncher
  fcl2zl    FoldCraftLauncher → Zalith Launcher 2
  api       启动 HTTP API 服务器

选项:
  --include-directions     将 FCL 方向控制近似为 ZL 按钮网格
  --lossless, --no-drop    替换不支持的控件而非丢弃
  --absolute-as-percentage 将绝对 dp 尺寸转为百分比尺寸
  --strict                 不支持的字段报错而非警告
  --compact                输出紧凑 JSON
  --strip-meta             去除元数据
  --usable                 ZL→FCL 安全可用模式
  --aspect 16/9            屏幕宽高比（默认 1.778）
  --host HOST              API 服务器地址
  --port PORT              API 服务器端口
```

## API 模式

```bash
# 启动服务器
./dist/cc api --host 0.0.0.0 --port 8000

# POST /convert
curl -X POST http://localhost:8000/convert \
  -H 'Content-Type: application/json' \
  -d '{"mode":"zl2fcl","data":{...}}'

# 健康检查
curl http://localhost:8000/health
```
