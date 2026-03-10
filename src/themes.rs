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
