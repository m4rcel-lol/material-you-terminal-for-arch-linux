//! Theme model and `ThemeManager`.
//!
//! Themes are stored as JSON files in `~/.config/myterminal/themes/`.  The
//! six built-in themes are written to disk if the directory is empty.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;

// ---------------------------------------------------------------------------
// Theme struct
// ---------------------------------------------------------------------------

/// A complete terminal colour theme.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    /// Hex colour for the terminal background (e.g. `"#121212"`).
    pub background: String,
    /// Hex colour for default foreground text.
    pub foreground: String,
    /// Cursor colour.
    pub cursor: String,
    /// Cursor foreground (text drawn on top of cursor block).
    pub cursor_foreground: String,
    /// Selection background colour.
    pub selection_background: String,
    /// Selection foreground colour.
    pub selection_foreground: String,
    /// Header bar / window surface colour.
    pub surface: String,
    /// Accent / tonal colour used for UI highlights.
    pub accent: String,
    /// 16 ANSI colour entries (indices 0-15).
    pub ansi: Vec<String>,
}

impl Default for Theme {
    fn default() -> Self {
        builtin_themes()
            .get("material_dark")
            .cloned()
            .unwrap_or_else(|| Theme {
                name: "Default".into(),
                background: "#121212".into(),
                foreground: "#E0E0E0".into(),
                cursor: "#BB86FC".into(),
                cursor_foreground: "#121212".into(),
                selection_background: "#3700B3".into(),
                selection_foreground: "#FFFFFF".into(),
                surface: "#1E1E2E".into(),
                accent: "#BB86FC".into(),
                ansi: ansi_dark(),
            })
    }
}

// ---------------------------------------------------------------------------
// Built-in theme registry
// ---------------------------------------------------------------------------

static BUILTIN: OnceLock<HashMap<String, Theme>> = OnceLock::new();

/// Return a reference to the built-in themes map (initialised once).
pub fn builtin_themes() -> &'static HashMap<String, Theme> {
    BUILTIN.get_or_init(make_builtin_themes)
}

fn make_builtin_themes() -> HashMap<String, Theme> {
    let mut m = HashMap::new();

    m.insert("material_dark".into(), Theme {
        name: "Material Dark".into(),
        background: "#121212".into(),
        foreground: "#E0E0E0".into(),
        cursor: "#BB86FC".into(),
        cursor_foreground: "#121212".into(),
        selection_background: "#3700B3".into(),
        selection_foreground: "#FFFFFF".into(),
        surface: "#1E1E2E".into(),
        accent: "#BB86FC".into(),
        ansi: ansi_dark(),
    });

    m.insert("material_light".into(), Theme {
        name: "Material Light".into(),
        background: "#FAFAFA".into(),
        foreground: "#1C1B1F".into(),
        cursor: "#6750A4".into(),
        cursor_foreground: "#FAFAFA".into(),
        selection_background: "#D0BCFF".into(),
        selection_foreground: "#21005D".into(),
        surface: "#FFFBFE".into(),
        accent: "#6750A4".into(),
        ansi: ansi_light(),
    });

    m.insert("amoled_black".into(), Theme {
        name: "AMOLED Black".into(),
        background: "#000000".into(),
        foreground: "#F8F8F2".into(),
        cursor: "#CF6679".into(),
        cursor_foreground: "#000000".into(),
        selection_background: "#44475A".into(),
        selection_foreground: "#F8F8F2".into(),
        surface: "#0D0D0D".into(),
        accent: "#CF6679".into(),
        ansi: vec![
            "#000000".into(), "#FF5555".into(), "#50FA7B".into(), "#F1FA8C".into(),
            "#BD93F9".into(), "#FF79C6".into(), "#8BE9FD".into(), "#BFBFBF".into(),
            "#4D4D4D".into(), "#FF6E6E".into(), "#69FF94".into(), "#FFFFA5".into(),
            "#D6ACFF".into(), "#FF92DF".into(), "#A4FFFF".into(), "#FFFFFF".into(),
        ],
    });

    m.insert("solarized_dark".into(), Theme {
        name: "Solarized Dark".into(),
        background: "#002B36".into(),
        foreground: "#839496".into(),
        cursor: "#268BD2".into(),
        cursor_foreground: "#002B36".into(),
        selection_background: "#073642".into(),
        selection_foreground: "#93A1A1".into(),
        surface: "#073642".into(),
        accent: "#268BD2".into(),
        ansi: vec![
            "#073642".into(), "#DC322F".into(), "#859900".into(), "#B58900".into(),
            "#268BD2".into(), "#D33682".into(), "#2AA198".into(), "#EEE8D5".into(),
            "#002B36".into(), "#CB4B16".into(), "#586E75".into(), "#657B83".into(),
            "#839496".into(), "#6C71C4".into(), "#93A1A1".into(), "#FDF6E3".into(),
        ],
    });

    m.insert("dracula".into(), Theme {
        name: "Dracula".into(),
        background: "#282A36".into(),
        foreground: "#F8F8F2".into(),
        cursor: "#FF79C6".into(),
        cursor_foreground: "#282A36".into(),
        selection_background: "#44475A".into(),
        selection_foreground: "#F8F8F2".into(),
        surface: "#21222C".into(),
        accent: "#BD93F9".into(),
        ansi: vec![
            "#21222C".into(), "#FF5555".into(), "#50FA7B".into(), "#F1FA8C".into(),
            "#BD93F9".into(), "#FF79C6".into(), "#8BE9FD".into(), "#F8F8F2".into(),
            "#6272A4".into(), "#FF6E6E".into(), "#69FF94".into(), "#FFFFA5".into(),
            "#D6ACFF".into(), "#FF92DF".into(), "#A4FFFF".into(), "#FFFFFF".into(),
        ],
    });

    m.insert("nord".into(), Theme {
        name: "Nord".into(),
        background: "#2E3440".into(),
        foreground: "#D8DEE9".into(),
        cursor: "#88C0D0".into(),
        cursor_foreground: "#2E3440".into(),
        selection_background: "#4C566A".into(),
        selection_foreground: "#ECEFF4".into(),
        surface: "#3B4252".into(),
        accent: "#88C0D0".into(),
        ansi: vec![
            "#3B4252".into(), "#BF616A".into(), "#A3BE8C".into(), "#EBCB8B".into(),
            "#81A1C1".into(), "#B48EAD".into(), "#88C0D0".into(), "#E5E9F0".into(),
            "#4C566A".into(), "#BF616A".into(), "#A3BE8C".into(), "#EBCB8B".into(),
            "#81A1C1".into(), "#B48EAD".into(), "#8FBCBB".into(), "#ECEFF4".into(),
        ],
    });

    m.insert("gruvbox_dark".into(), Theme {
        name: "Gruvbox Dark".into(),
        background: "#282828".into(),
        foreground: "#EBDBB2".into(),
        cursor: "#FE8019".into(),
        cursor_foreground: "#282828".into(),
        selection_background: "#504945".into(),
        selection_foreground: "#EBDBB2".into(),
        surface: "#3C3836".into(),
        accent: "#FE8019".into(),
        ansi: vec![
            "#282828".into(), "#CC241D".into(), "#98971A".into(), "#D79921".into(),
            "#458588".into(), "#B16286".into(), "#689D6A".into(), "#A89984".into(),
            "#928374".into(), "#FB4934".into(), "#B8BB26".into(), "#FABD2F".into(),
            "#83A598".into(), "#D3869B".into(), "#8EC07C".into(), "#EBDBB2".into(),
        ],
    });

    m.insert("gruvbox_light".into(), Theme {
        name: "Gruvbox Light".into(),
        background: "#FBF1C7".into(),
        foreground: "#3C3836".into(),
        cursor: "#AF3A03".into(),
        cursor_foreground: "#FBF1C7".into(),
        selection_background: "#EBDBB2".into(),
        selection_foreground: "#3C3836".into(),
        surface: "#F9F5D7".into(),
        accent: "#AF3A03".into(),
        ansi: vec![
            "#FBF1C7".into(), "#CC241D".into(), "#98971A".into(), "#D79921".into(),
            "#458588".into(), "#B16286".into(), "#689D6A".into(), "#7C6F64".into(),
            "#928374".into(), "#9D0006".into(), "#79740E".into(), "#B57614".into(),
            "#076678".into(), "#8F3F71".into(), "#427B58".into(), "#3C3836".into(),
        ],
    });

    m.insert("catppuccin_mocha".into(), Theme {
        name: "Catppuccin Mocha".into(),
        background: "#1E1E2E".into(),
        foreground: "#CDD6F4".into(),
        cursor: "#F5E0DC".into(),
        cursor_foreground: "#1E1E2E".into(),
        selection_background: "#585B70".into(),
        selection_foreground: "#CDD6F4".into(),
        surface: "#181825".into(),
        accent: "#F5C2E7".into(),
        ansi: vec![
            "#45475A".into(), "#F38BA8".into(), "#A6E3A1".into(), "#F9E2AF".into(),
            "#89B4FA".into(), "#F5C2E7".into(), "#94E2D5".into(), "#BAC2DE".into(),
            "#585B70".into(), "#F38BA8".into(), "#A6E3A1".into(), "#F9E2AF".into(),
            "#89B4FA".into(), "#F5C2E7".into(), "#94E2D5".into(), "#A6ADC8".into(),
        ],
    });

    m.insert("catppuccin_latte".into(), Theme {
        name: "Catppuccin Latte".into(),
        background: "#EFF1F5".into(),
        foreground: "#4C4F69".into(),
        cursor: "#DC8A78".into(),
        cursor_foreground: "#EFF1F5".into(),
        selection_background: "#ACB0BE".into(),
        selection_foreground: "#4C4F69".into(),
        surface: "#E6E9EF".into(),
        accent: "#8839EF".into(),
        ansi: vec![
            "#5C5F77".into(), "#D20F39".into(), "#40A02B".into(), "#DF8E1D".into(),
            "#1E66F5".into(), "#EA76CB".into(), "#179299".into(), "#ACB0BE".into(),
            "#6C6F85".into(), "#D20F39".into(), "#40A02B".into(), "#DF8E1D".into(),
            "#1E66F5".into(), "#EA76CB".into(), "#179299".into(), "#BCC0CC".into(),
        ],
    });

    m.insert("tokyo_night".into(), Theme {
        name: "Tokyo Night".into(),
        background: "#1A1B26".into(),
        foreground: "#C0CAF5".into(),
        cursor: "#C0CAF5".into(),
        cursor_foreground: "#1A1B26".into(),
        selection_background: "#283457".into(),
        selection_foreground: "#C0CAF5".into(),
        surface: "#16161E".into(),
        accent: "#7AA2F7".into(),
        ansi: vec![
            "#15161E".into(), "#F7768E".into(), "#9ECE6A".into(), "#E0AF68".into(),
            "#7AA2F7".into(), "#BB9AF7".into(), "#7DCFFF".into(), "#A9B1D6".into(),
            "#414868".into(), "#F7768E".into(), "#9ECE6A".into(), "#E0AF68".into(),
            "#7AA2F7".into(), "#BB9AF7".into(), "#7DCFFF".into(), "#C0CAF5".into(),
        ],
    });

    m.insert("tokyo_night_storm".into(), Theme {
        name: "Tokyo Night Storm".into(),
        background: "#24283B".into(),
        foreground: "#C0CAF5".into(),
        cursor: "#C0CAF5".into(),
        cursor_foreground: "#24283B".into(),
        selection_background: "#364A82".into(),
        selection_foreground: "#C0CAF5".into(),
        surface: "#1F2335".into(),
        accent: "#7AA2F7".into(),
        ansi: vec![
            "#1D202F".into(), "#F7768E".into(), "#9ECE6A".into(), "#E0AF68".into(),
            "#7AA2F7".into(), "#BB9AF7".into(), "#7DCFFF".into(), "#A9B1D6".into(),
            "#414868".into(), "#F7768E".into(), "#9ECE6A".into(), "#E0AF68".into(),
            "#7AA2F7".into(), "#BB9AF7".into(), "#7DCFFF".into(), "#C0CAF5".into(),
        ],
    });

    m.insert("one_dark".into(), Theme {
        name: "One Dark".into(),
        background: "#282C34".into(),
        foreground: "#ABB2BF".into(),
        cursor: "#528BFF".into(),
        cursor_foreground: "#282C34".into(),
        selection_background: "#3E4451".into(),
        selection_foreground: "#ABB2BF".into(),
        surface: "#21252B".into(),
        accent: "#61AFEF".into(),
        ansi: vec![
            "#282C34".into(), "#E06C75".into(), "#98C379".into(), "#E5C07B".into(),
            "#61AFEF".into(), "#C678DD".into(), "#56B6C2".into(), "#ABB2BF".into(),
            "#5C6370".into(), "#E06C75".into(), "#98C379".into(), "#E5C07B".into(),
            "#61AFEF".into(), "#C678DD".into(), "#56B6C2".into(), "#FFFFFF".into(),
        ],
    });

    m.insert("one_light".into(), Theme {
        name: "One Light".into(),
        background: "#FAFAFA".into(),
        foreground: "#383A42".into(),
        cursor: "#526FFF".into(),
        cursor_foreground: "#FAFAFA".into(),
        selection_background: "#E5E5E6".into(),
        selection_foreground: "#383A42".into(),
        surface: "#F0F0F1".into(),
        accent: "#4078F2".into(),
        ansi: vec![
            "#FAFAFA".into(), "#CA1243".into(), "#50A14F".into(), "#C18401".into(),
            "#4078F2".into(), "#A626A4".into(), "#0184BC".into(), "#383A42".into(),
            "#A0A1A7".into(), "#CA1243".into(), "#50A14F".into(), "#C18401".into(),
            "#4078F2".into(), "#A626A4".into(), "#0184BC".into(), "#090A0B".into(),
        ],
    });

    m.insert("monokai_pro".into(), Theme {
        name: "Monokai Pro".into(),
        background: "#2D2A2E".into(),
        foreground: "#FCFCFA".into(),
        cursor: "#FFD866".into(),
        cursor_foreground: "#2D2A2E".into(),
        selection_background: "#5B595C".into(),
        selection_foreground: "#FCFCFA".into(),
        surface: "#221F22".into(),
        accent: "#FFD866".into(),
        ansi: vec![
            "#2D2A2E".into(), "#FF6188".into(), "#A9DC76".into(), "#FFD866".into(),
            "#FC9867".into(), "#AB9DF2".into(), "#78DCE8".into(), "#FCFCFA".into(),
            "#727072".into(), "#FF6188".into(), "#A9DC76".into(), "#FFD866".into(),
            "#FC9867".into(), "#AB9DF2".into(), "#78DCE8".into(), "#FFFFFF".into(),
        ],
    });

    m.insert("github_dark".into(), Theme {
        name: "GitHub Dark".into(),
        background: "#0D1117".into(),
        foreground: "#C9D1D9".into(),
        cursor: "#58A6FF".into(),
        cursor_foreground: "#0D1117".into(),
        selection_background: "#1F6FEB".into(),
        selection_foreground: "#FFFFFF".into(),
        surface: "#161B22".into(),
        accent: "#58A6FF".into(),
        ansi: vec![
            "#484F58".into(), "#FF7B72".into(), "#3FB950".into(), "#D29922".into(),
            "#58A6FF".into(), "#BC8CFF".into(), "#39C5CF".into(), "#B1BAC4".into(),
            "#6E7681".into(), "#FFA198".into(), "#56D364".into(), "#E3B341".into(),
            "#79C0FF".into(), "#D2A8FF".into(), "#56D4DD".into(), "#F0F6FC".into(),
        ],
    });

    m.insert("github_light".into(), Theme {
        name: "GitHub Light".into(),
        background: "#FFFFFF".into(),
        foreground: "#24292F".into(),
        cursor: "#0969DA".into(),
        cursor_foreground: "#FFFFFF".into(),
        selection_background: "#0969DA".into(),
        selection_foreground: "#FFFFFF".into(),
        surface: "#F6F8FA".into(),
        accent: "#0969DA".into(),
        ansi: vec![
            "#24292F".into(), "#CF222E".into(), "#116329".into(), "#4D2D00".into(),
            "#0969DA".into(), "#8250DF".into(), "#1B7C83".into(), "#6E7781".into(),
            "#57606A".into(), "#A40E26".into(), "#1A7F37".into(), "#633C01".into(),
            "#218BFF".into(), "#A475F9".into(), "#3192AA".into(), "#8C959F".into(),
        ],
    });

    m
}

// ---------------------------------------------------------------------------
// ThemeManager
// ---------------------------------------------------------------------------

/// Manages all known themes (built-in + user-installed).
pub struct ThemeManager {
    themes: HashMap<String, Theme>,
    active_key: String,
}

impl ThemeManager {
    /// Create a new manager, loading built-ins and any user themes.
    pub fn new(initial_theme: &str) -> Self {
        let mut themes = builtin_themes().clone();

        // Merge user themes from ~/.config/myterminal/themes/
        if let Ok(user) = Self::load_user_themes() {
            themes.extend(user);
        }

        // Write built-in themes to disk on first run.
        let _ = Self::seed_theme_directory();

        let active_key = if themes.contains_key(initial_theme) {
            initial_theme.to_string()
        } else {
            "material_dark".to_string()
        };

        ThemeManager { themes, active_key }
    }

    /// Return the currently active theme.
    pub fn active(&self) -> &Theme {
        self.themes
            .get(&self.active_key)
            .or_else(|| self.themes.get("material_dark"))
            .expect("material_dark theme must always be present")
    }

    /// Switch the active theme by key.  Returns `true` on success.
    pub fn set_active(&mut self, key: &str) -> bool {
        if self.themes.contains_key(key) {
            self.active_key = key.to_string();
            true
        } else {
            false
        }
    }

    /// All available theme (key, display-name) pairs sorted by display name.
    pub fn theme_names(&self) -> Vec<(&str, &str)> {
        let mut pairs: Vec<_> = self
            .themes
            .iter()
            .map(|(k, v)| (k.as_str(), v.name.as_str()))
            .collect();
        pairs.sort_by_key(|&(_, name)| name);
        pairs
    }

    /// Key of the currently active theme.
    pub fn active_key(&self) -> &str {
        &self.active_key
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    fn themes_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from(".config"))
            .join("myterminal")
            .join("themes")
    }

    fn load_user_themes() -> Result<HashMap<String, Theme>, Box<dyn std::error::Error>> {
        let dir = Self::themes_dir();
        let mut themes = HashMap::new();
        if !dir.exists() {
            return Ok(themes);
        }
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                match std::fs::read_to_string(&path) {
                    Ok(contents) => match serde_json::from_str::<Theme>(&contents) {
                        Ok(theme) => {
                            let key = path
                                .file_stem()
                                .and_then(|s| s.to_str())
                                .unwrap_or("unknown")
                                .to_string();
                            themes.insert(key, theme);
                        }
                        Err(e) => log::warn!("Ignoring invalid theme {}: {e}", path.display()),
                    },
                    Err(e) => log::warn!("Cannot read theme {}: {e}", path.display()),
                }
            }
        }
        Ok(themes)
    }

    /// Write all built-in themes to the user themes directory if absent.
    fn seed_theme_directory() -> Result<(), Box<dyn std::error::Error>> {
        let dir = Self::themes_dir();
        std::fs::create_dir_all(&dir)?;
        for (key, theme) in builtin_themes() {
            let path = dir.join(format!("{key}.json"));
            if !path.exists() {
                let json = serde_json::to_string_pretty(theme)?;
                std::fs::write(&path, json)?;
            }
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// ANSI palette helpers
// ---------------------------------------------------------------------------

fn ansi_dark() -> Vec<String> {
    vec![
        "#1E1E2E".into(), "#F38BA8".into(), "#A6E3A1".into(), "#F9E2AF".into(),
        "#89B4FA".into(), "#CBA6F7".into(), "#89DCEB".into(), "#BAC2DE".into(),
        "#45475A".into(), "#F38BA8".into(), "#A6E3A1".into(), "#F9E2AF".into(),
        "#89B4FA".into(), "#CBA6F7".into(), "#89DCEB".into(), "#CDD6F4".into(),
    ]
}

fn ansi_light() -> Vec<String> {
    vec![
        "#FAFAFA".into(), "#B3261E".into(), "#386A20".into(), "#7D5700".into(),
        "#0061A4".into(), "#6750A4".into(), "#006874".into(), "#1C1B1F".into(),
        "#938F99".into(), "#B3261E".into(), "#386A20".into(), "#7D5700".into(),
        "#0061A4".into(), "#6750A4".into(), "#006874".into(), "#49454F".into(),
    ]
}
