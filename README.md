# MyTerminal

A **modern, production-ready terminal emulator** styled with **Material You (Material Design 3)** for Arch Linux and other GTK4-based Linux desktops.  
MyTerminal combines the raw power of a POSIX pseudo-terminal with a polished, contemporary UI that surpasses GNOME Console in aesthetics and feature set.

---

## ✨ Features

| Category | Details |
|----------|---------|
| **UI Framework** | Rust · GTK4 · Libadwaita (Material You) |
| **Terminal Engine** | VTE4 (vte-2.91-gtk4) |
| **Tabs** | Create, close, rename, duplicate, reorder with smooth animations |
| **Keyboard Shortcuts** | Fully customisable via `config.json` |
| **Themes** | 17 professional built-in themes + user-defined JSON themes |
| **Quick Theme Switcher** | One-click theme switching from header bar dropdown |
| **Settings Panel** | Libadwaita Preferences Window (fonts, opacity, cursor, bold text, hyperlinks) |
| **Command Palette** | VS Code-style fuzzy search (`Ctrl+Shift+P`) |
| **Search** | In-terminal text search (`Ctrl+Shift+F`) |
| **Context Menu** | Right-click → copy / paste / search / new tab |
| **Toast Notifications** | Non-intrusive confirmations with animations |
| **Scrollback Buffer** | Configurable (default 10,000 lines, unlimited supported) |
| **Transparency** | Per-terminal background opacity |
| **True Colour** | 24-bit colour + full Unicode support |
| **Clickable URLs** | Configurable hyperlink detection and highlighting |
| **Bold Text** | Configurable bold text rendering and bright color variants |
| **Zoom** | `Ctrl++` / `Ctrl+-` / `Ctrl+0` |
| **Modern UI** | Material Design 3 with smooth transitions, shadows, and hover effects |

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

17 professional built-in themes are included and also written to `~/.config/myterminal/themes/` on first run:

### Dark Themes
| Theme | Style |
|-------|-------|
| `material_dark` | Material You dark (default) |
| `amoled_black` | Pure black OLED optimized |
| `solarized_dark` | Classic Solarized Dark |
| `dracula` | Popular Dracula theme |
| `nord` | Nordic-inspired Nord |
| `gruvbox_dark` | Retro groove Gruvbox |
| `catppuccin_mocha` | Catppuccin Mocha |
| `tokyo_night` | Tokyo Night default |
| `tokyo_night_storm` | Tokyo Night Storm variant |
| `one_dark` | Atom One Dark |
| `monokai_pro` | Professional Monokai |
| `github_dark` | GitHub Dark theme |

### Light Themes
| Theme | Style |
|-------|-------|
| `material_light` | Material You light |
| `gruvbox_light` | Light Gruvbox variant |
| `catppuccin_latte` | Catppuccin Latte |
| `one_light` | Atom One Light |
| `github_light` | GitHub Light theme |

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

Restart MyTerminal (or switch to the theme in settings/quick switcher) to load new themes.

### Quick Theme Switching

Click the palette icon (🎨) in the header bar to instantly switch between all available themes without opening settings. Theme changes are applied immediately and persisted automatically.

---

## 🎨 Enhanced UI

MyTerminal features a polished, modern interface with Material Design 3:

- **Smooth Animations**: All UI elements feature 200ms cubic-bezier transitions
- **Enhanced Tab Bar**: Rounded tabs with hover effects, shadows, and smooth transitions
- **Modern Header Bar**: Elevated design with subtle shadows and button hover states
- **Styled Scrollbars**: Custom scrollbars with smooth hover animations
- **Beautiful Command Palette**: Enhanced with proper spacing, shadows, and visual hierarchy
- **Toast Notifications**: Modern rounded toasts with smooth animations
- **Window Decorations**: Elegant shadows that adapt when maximized
- **Improved Buttons**: Gradient backgrounds for suggested/destructive actions
- **Menu Polish**: Rounded menu items with smooth hover transitions
- **Accessibility**: High contrast support and keyboard navigation throughout

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
  "bell": false,
  "font_ligatures": false,
  "allow_bold": true,
  "bold_is_bright": true,
  "allow_hyperlinks": true,
  "audible_bell": false
}
```

### Advanced Settings

- **`allow_bold`**: Enable/disable bold text rendering in the terminal
- **`bold_is_bright`**: Use bright color variants for bold text
- **`allow_hyperlinks`**: Automatic URL detection and clickable links
- **`audible_bell`**: System sound on terminal bell (BEL character)
- **`font_ligatures`**: Enable font ligatures (requires ligature-capable font)

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
