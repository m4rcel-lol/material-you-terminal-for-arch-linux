//! Settings model — serialised to / deserialised from
//! `~/.config/myterminal/config.json`.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Sub-structures
// ---------------------------------------------------------------------------

/// Terminal cursor style.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CursorShape {
    Block,
    Underline,
    IBeam,
}

impl Default for CursorShape {
    fn default() -> Self {
        CursorShape::Block
    }
}

impl CursorShape {
    /// Convert to the VTE enum value.
    pub fn to_vte(&self) -> vte4::CursorShape {
        match self {
            CursorShape::Block => vte4::CursorShape::Block,
            CursorShape::Underline => vte4::CursorShape::Underline,
            CursorShape::IBeam => vte4::CursorShape::Ibeam,
        }
    }
}

/// Customisable keyboard shortcuts (stored as GTK accelerator strings).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keybindings {
    pub new_tab: String,
    pub close_tab: String,
    pub next_tab: String,
    pub prev_tab: String,
    pub search: String,
    pub command_palette: String,
    pub zoom_in: String,
    pub zoom_out: String,
    pub zoom_reset: String,
    pub split_horizontal: String,
    pub split_vertical: String,
    pub copy: String,
    pub paste: String,
}

impl Default for Keybindings {
    fn default() -> Self {
        Keybindings {
            new_tab: "<Control><Shift>t".into(),
            close_tab: "<Control><Shift>w".into(),
            next_tab: "<Control>Tab".into(),
            prev_tab: "<Control><Shift>Tab".into(),
            search: "<Control><Shift>f".into(),
            command_palette: "<Control><Shift>p".into(),
            zoom_in: "<Control>plus".into(),
            zoom_out: "<Control>minus".into(),
            zoom_reset: "<Control>0".into(),
            split_horizontal: "<Control><Shift>d".into(),
            split_vertical: "<Control>d".into(),
            copy: "<Control><Shift>c".into(),
            paste: "<Control><Shift>v".into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Main settings struct
// ---------------------------------------------------------------------------

/// Full application settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Theme name (matches a JSON file in the themes directory).
    pub theme: String,
    /// Terminal font family.
    pub font_family: String,
    /// Terminal font size in points.
    pub font_size: f64,
    /// Default shell (e.g. `/bin/bash`).  `None` = read `$SHELL`.
    pub shell: Option<String>,
    /// Number of scrollback lines (-1 = unlimited).
    pub scrollback_lines: i64,
    /// Whether the cursor blinks.
    pub cursor_blink: bool,
    /// Terminal background opacity (0.0 – 1.0).
    pub transparency: f64,
    /// Inner padding (px) around the terminal widget.
    pub padding: i32,
    /// Startup working directory.  `None` = home directory.
    pub startup_directory: Option<String>,
    /// Cursor shape.
    pub cursor_shape: CursorShape,
    /// Whether to play a bell sound.
    pub bell: bool,
    /// Enable font ligatures.
    pub font_ligatures: bool,
    /// Keyboard shortcut customisation.
    pub keybindings: Keybindings,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            theme: "material_dark".into(),
            font_family: "Monospace".into(),
            font_size: 13.0,
            shell: None,
            scrollback_lines: 10_000,
            cursor_blink: true,
            transparency: 1.0,
            padding: 8,
            startup_directory: None,
            cursor_shape: CursorShape::default(),
            bell: false,
            font_ligatures: false,
            keybindings: Keybindings::default(),
        }
    }
}

impl Settings {
    // -----------------------------------------------------------------------
    // Persistence helpers
    // -----------------------------------------------------------------------

    /// Path to the config file: `~/.config/myterminal/config.json`.
    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from(".config"))
            .join("myterminal")
            .join("config.json")
    }

    /// Load settings from disk; falls back to `Default` if the file is absent.
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::config_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let contents = std::fs::read_to_string(&path)?;
        let settings: Self = serde_json::from_str(&contents)?;
        log::debug!("Settings loaded from {}", path.display());
        Ok(settings)
    }

    /// Persist settings to disk, creating the directory if necessary.
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, json)?;
        log::debug!("Settings saved to {}", path.display());
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Convenience accessors
    // -----------------------------------------------------------------------

    /// Resolve the shell to use, preferring `$SHELL` over the stored value.
    pub fn resolved_shell(&self) -> String {
        self.shell
            .clone()
            .unwrap_or_else(|| std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".into()))
    }
}
