# MyTerminal

A **modern, production-ready terminal emulator** styled with **Material You (Material Design 3)** for Arch Linux and other GTK4-based Linux desktops.  
MyTerminal combines the raw power of a POSIX pseudo-terminal with a polished, contemporary UI that surpasses GNOME Console in aesthetics and feature set.

---

## ✨ Features

| Category | Details |
|----------|---------|
| **UI Framework** | Rust · GTK4 · Libadwaita (Material You) |
| **Terminal Engine** | VTE4 (vte-2.91-gtk4) |
| **Tabs** | Create, close, rename, duplicate, reorder |
| **Keyboard Shortcuts** | Fully customisable via `config.json` |
| **Themes** | 6 built-in themes + user-defined JSON themes |
| **Settings Panel** | Libadwaita Preferences Window (fonts, opacity, cursor …) |
| **Command Palette** | VS Code-style fuzzy search (`Ctrl+Shift+P`) |
| **Search** | In-terminal text search (`Ctrl+Shift+F`) |
| **Context Menu** | Right-click → copy / paste / search / new tab |
| **Toast Notifications** | Non-intrusive confirmations (copy, errors, …) |
| **Scrollback Buffer** | Configurable (default 10 000 lines) |
| **Transparency** | Per-terminal background opacity |
| **True Colour** | 24-bit colour + full Unicode support |
| **Clickable URLs** | Hyperlink detection in terminal output |
| **Zoom** | `Ctrl++` / `Ctrl+-` / `Ctrl+0` |

---

## 📦 Installation (Arch Linux)

### 1 · Install system dependencies

```bash
sudo pacman -S \
    base-devel \
    rust \
    pkg-config \
    gtk4 \
    libadwaita \
    vte4
```

### 2 · Clone and build

```bash
git clone https://github.com/m4rcel-lol/material-you-terminal-for-arch-linux.git
cd material-you-terminal-for-arch-linux
cargo build --release
```

### 3 · Run

```bash
cargo run --release
# or, after installation:
./target/release/myterminal
```

### 4 · (Optional) Install system-wide

```bash
sudo install -Dm755 target/release/myterminal /usr/local/bin/myterminal
sudo install -Dm644 data/com.myterminal.app.desktop \
    /usr/share/applications/com.myterminal.app.desktop
```

---

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Shift+T` | New tab |
| `Ctrl+Shift+W` | Close current tab |
| `Ctrl+Tab` | Next tab |
| `Ctrl+Shift+Tab` | Previous tab |
| `Ctrl+Shift+F` | Toggle search bar |
| `Ctrl+Shift+P` | Open command palette |
| `Ctrl+Shift+C` | Copy selection |
| `Ctrl+Shift+V` | Paste clipboard |
| `Ctrl++` | Zoom in |
| `Ctrl+-` | Zoom out |
| `Ctrl+0` | Reset zoom |
| `Ctrl+,` | Open settings |
| `Ctrl+Shift+D` | Split horizontal (new tab) |
| `Ctrl+D` | Split vertical (new tab) |

---

## 🎨 Themes

Six built-in themes are included and also written to `~/.config/myterminal/themes/` on first run:

| Theme | Style |
|-------|-------|
| `material_dark` | Material You dark (default) |
| `material_light` | Material You light |
| `amoled_black` | Pure black OLED |
| `solarized_dark` | Solarized Dark |
| `dracula` | Dracula |
| `nord` | Nord |

### Custom themes

Add a JSON file to `~/.config/myterminal/themes/`:

```json
{
  "name": "My Custom Theme",
  "background": "#1a1b26",
  "foreground": "#c0caf5",
  "cursor": "#f7768e",
  "cursor_foreground": "#1a1b26",
  "selection_background": "#283457",
  "selection_foreground": "#c0caf5",
  "surface": "#16161e",
  "accent": "#7aa2f7",
  "ansi": [
    "#15161e", "#f7768e", "#9ece6a", "#e0af68",
    "#7aa2f7", "#bb9af7", "#7dcfff", "#a9b1d6",
    "#414868", "#f7768e", "#9ece6a", "#e0af68",
    "#7aa2f7", "#bb9af7", "#7dcfff", "#c0caf5"
  ]
}
```

Restart MyTerminal (or switch to the theme in settings) to load new themes.

---

## ⚙️ Configuration

Settings are stored at `~/.config/myterminal/config.json`.  
All changes made in the Settings panel are persisted automatically on close.

Key options:

```json
{
  "theme": "material_dark",
  "font_family": "JetBrains Mono",
  "font_size": 13.0,
  "shell": null,
  "scrollback_lines": 10000,
  "cursor_blink": true,
  "transparency": 1.0,
  "padding": 8,
  "cursor_shape": "block",
  "bell": false
}
```

---

## 📁 Project Structure

```
myterminal/
├── src/
│   ├── main.rs            Entry point + logging
│   ├── app.rs             GApplication bootstrap & accelerators
│   ├── window.rs          Main window, GActions, CSS
│   ├── terminal.rs        VTE terminal widget wrapper
│   ├── tabs.rs            Tab management (AdwTabView)
│   ├── themes.rs          Theme model & ThemeManager
│   ├── settings.rs        Settings model & JSON persistence
│   ├── commands.rs        Command palette
│   └── ui/
│       ├── mod.rs
│       ├── headerbar.rs   Custom AdwHeaderBar
│       ├── settings_panel.rs  AdwPreferencesWindow
│       └── dialogs.rs     About, confirm, rename, search dialogs
├── assets/
│   └── themes/            Built-in theme JSON files
├── config/
│   └── default_config.json
├── data/
│   └── com.myterminal.app.desktop
├── Cargo.toml
└── README.md
```

---

## 🔧 Development

```bash
# Debug build (faster compile)
cargo build

# Release build (optimised)
cargo build --release

# Run with verbose logging
RUST_LOG=debug cargo run

# Check for errors without producing a binary
cargo check
```

Logs are written to stderr (use `RUST_LOG=info myterminal 2>~/.local/share/myterminal/myterminal.log` for file logging).

---

## 📄 License

GPL-3.0 — see [LICENSE](LICENSE).
