# control-converter

Convert control-layout JSON between [Zalith Launcher 2](https://github.com/ZalithLauncher/ZalithLauncher2) and [FoldCraftLauncher](https://github.com/FCL-Team/FoldCraftLauncher).

[![License: MIT](https://img.shields.io/badge/license-MIT-green)](LICENSE)
[![Python](https://img.shields.io/badge/python-3.8%2B-blue)](https://www.python.org/)

## Quick Start

```bash
# ZL -> FCL
python cc.py zl2fcl zl_layout.json fcl_layout.json

# FCL -> ZL
python cc.py fcl2zl fcl_layout.json zl_layout.json

# auto-detect format
python cc.py auto input.json output.json
```

Zero dependencies. Python 3.8+ standard library only.

## Install

```bash
git clone https://github.com/leemwood/control-converter.git
cd control-converter
```

No `pip install` needed. Just run `python cc.py`.

## What It Does

Zalith Launcher 2 and FoldCraftLauncher use completely different JSON schemas for their on-screen control layouts -- different key codes, color formats, size units, and event models. This tool translates between the two, handling:

- **Key mapping** -- bi-directional GLFW key names <-> FCL integer keycodes (100+ keys, 3 mouse buttons, scroll events)
- **Color conversion** -- ZL Compose Color (signed Long packed sRGB) <-> FCL ARGB int, with alpha blending
- **Size conversion** -- ZL dp/percentage/wrap_content <-> FCL ABSOLUTE/PERCENTAGE with per-mille scaling, including wrap_content dp estimation
- **Style mapping** -- bidirectional button style conversion (text, border, fill, corner radius, pressed state)
- **Event translation** -- key presses, launcher events (scroll, IME toggle, menu), send_text, layer visibility toggles
- **Layer preservation** -- ZL layers -> FCL view groups with group-level visibility and order
- **Direction controls** -- FCL rocker/direction pad <-> ZL joystick + button grids
- **Roundtrip metadata** -- embeds `_control_converter` metadata in output so the original layout can be recovered

Modes that can't map 1:1 use documented fallbacks with substitution tracking, printed to stderr.

## Features

| Feature | CLI | API |
|---------|-----|-----|
| ZL -> FCL | `zl2fcl` | `/convert` |
| FCL -> ZL | `fcl2zl` | `/convert` |
| Auto-detect | `auto` | `/convert` |
| Direction controls | `--include-directions` | `includeDirections` |
| Lossless mode | `--lossless` | `lossless` |
| Absolute -> percentage | `--absolute-as-percentage` | `absoluteAsPercentage` |
| Strip metadata | `--strip-meta` | `stripMeta` |
| Usable safe mode | `--usable` | `usable` |
| JSON comment support | -- | `/convert-file` |
| Health check | -- | `GET /health` |

## CLI Usage

```bash
python cc.py <mode> <input> <output> [options]
```

### Modes

| Mode | Direction |
|------|-----------|
| `zl2fcl` | Zalith Launcher 2 -> FoldCraftLauncher |
| `fcl2zl` | FoldCraftLauncher -> Zalith Launcher 2 |
| `auto` | Detect format and convert |
| `api` | Start HTTP API server |

### Options

| Flag | Effect |
|------|--------|
| `--include-directions` | Approximate FCL direction controls as ZL button grids |
| `--lossless` | Substitute unsupported controls instead of dropping them; also converts FCL directions |
| `--absolute-as-percentage` | Convert FCL absolute dp sizes to ZL percentage sizes |
| `--strict` | Fail on unsupported fields instead of warning |
| `--compact` | Output compact JSON (no whitespace) |
| `--strip-meta` | Remove `_control_converter` metadata from output |
| `--usable` | ZL->FCL structural safe mode; remove only large blank blockers |
| `--aspect 16/9` | Screen aspect ratio for direction approximation (default: 1.778) |

## API Server

Start the server:

```bash
python cc.py api --host 0.0.0.0 --port 8000
```

### Endpoints

**POST /convert** -- Convert layout in request body.

```json
{
  "mode": "zl2fcl",
  "data": { ... },
  "strict": false,
  "stripMeta": false
}
```

Response:
```json
{ "ok": true, "data": { ... } }
```

**POST /convert-file** -- Convert uploaded JSON file. Supports `//` and `/* */` comments.

```
POST /convert-file?mode=zl2fcl&strict=false
Content-Type: application/json

{ ... layout with comments ... }
```

**GET /health** -- Health check.

```json
{ "ok": true }
```

## Limitations

- ZL `wrap_content` sizes have no FCL equivalent; estimated from text length and font size
- ZL `hideWhenMouse` / `hideWhenGamepad` layer flags have no FCL equivalent; skipped with warning
- FCL side mouse buttons 4-8 map to scroll events as fallback
- FCL `KEY_KPCOMMA` has no exact GLFW key; falls back to keypad decimal

All substitutions are tracked, documented with reasons, and printed to stderr.

## License

MIT -- see [LICENSE](LICENSE).
