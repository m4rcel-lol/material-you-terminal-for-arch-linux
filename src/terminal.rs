//! VTE terminal widget wrapper.
//!
//! `TerminalWidget` owns a single `vte4::Terminal`, applies colour themes,
//! spawns the shell process, and exposes higher-level helpers used by the
//! tab / window layers.

use crate::settings::Settings;
use crate::themes::Theme;
use pango::FontDescription;
use vte4::prelude::*;

/// A single terminal instance (one VTE widget + one shell process).
#[derive(Clone)]
pub struct TerminalWidget {
    pub terminal: vte4::Terminal,
}

impl TerminalWidget {
    /// Create a new terminal widget configured from `settings`.
    pub fn new(settings: &Settings) -> Self {
        let terminal = vte4::Terminal::new();

        // Basic configuration.
        terminal.set_scrollback_lines(settings.scrollback_lines);
        terminal.set_cursor_blink_mode(if settings.cursor_blink {
            vte4::CursorBlinkMode::On
        } else {
            vte4::CursorBlinkMode::Off
        });
        terminal.set_cursor_shape(settings.cursor_shape.to_vte());
        terminal.set_mouse_autohide(true);
        terminal.set_allow_hyperlink(settings.allow_hyperlinks);
        // Note: set_allow_bold was removed from VTE4; bold rendering is now always enabled
        terminal.set_bold_is_bright(settings.bold_is_bright);
        terminal.set_audible_bell(settings.audible_bell);

        // Font.
        let font_str = format!("{} {}", settings.font_family, settings.font_size as i32);
        let font_desc = FontDescription::from_string(&font_str);
        terminal.set_font(Some(&font_desc));

        // Inner padding via CSS.
        let provider = gtk4::CssProvider::new();
        provider.load_from_string(&format!(
            "vte-terminal {{ padding: {}px; }}",
            settings.padding
        ));
        if let Some(display) = gdk4::Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }

        TerminalWidget { terminal }
    }

    /// Apply a colour theme with the given background opacity (0.0–1.0).
    ///
    /// VTE `set_colors` argument order: foreground, background, palette.
    pub fn apply_theme(&self, theme: &Theme, opacity: f64) {
        let alpha = opacity.clamp(0.0, 1.0) as f32;

        let fg = parse_rgba(&theme.foreground);
        let bg = parse_rgba_with_alpha(&theme.background, alpha);
        let cursor = parse_rgba(&theme.cursor);
        let cursor_fg = parse_rgba(&theme.cursor_foreground);
        let sel_bg = parse_rgba(&theme.selection_background);
        let sel_fg = parse_rgba(&theme.selection_foreground);

        // Build the 16-colour ANSI palette as a slice of references.
        let palette_owned: Vec<gdk4::RGBA> = theme.ansi.iter().map(|c| parse_rgba(c)).collect();
        let palette_refs: Vec<&gdk4::RGBA> = palette_owned.iter().collect();

        // Note: VTE's argument order is (foreground, background, palette).
        self.terminal
            .set_colors(Some(&fg), Some(&bg), &palette_refs);
        self.terminal.set_color_cursor(Some(&cursor));
        self.terminal.set_color_cursor_foreground(Some(&cursor_fg));
        self.terminal.set_color_highlight(Some(&sel_bg));
        self.terminal.set_color_highlight_foreground(Some(&sel_fg));
    }

    /// Spawn the configured shell inside this terminal.
    pub fn spawn_shell(&self, settings: &Settings) {
        let shell = settings.resolved_shell();

        // VTE's `spawn_async` requires `Option<&str>` for the working directory.
        let home = dirs::home_dir();
        let workdir: Option<String> = settings
            .startup_directory
            .clone()
            .or_else(|| home.map(|p| p.to_string_lossy().into_owned()));

        let term_clone = self.terminal.clone();
        self.terminal.spawn_async(
            vte4::PtyFlags::DEFAULT,
            workdir.as_deref(),
            &[shell.as_str()],
            &[] as &[&str],
            glib::SpawnFlags::DEFAULT,
            || {},
            -1,
            None::<&gio::Cancellable>,
            move |result| {
                if let Err(ref e) = result {
                    log::error!("Failed to spawn shell: {e}");
                    let msg = format!(
                        "\r\n\x1b[1;31mMyTerminal: could not start shell — {e}\x1b[0m\r\n"
                    );
                    term_clone.feed(msg.as_bytes());
                }
            },
        );
    }

    /// Copy selected text to the clipboard.
    pub fn copy(&self) {
        self.terminal.copy_clipboard_format(vte4::Format::Text);
    }

    /// Paste from the clipboard.
    pub fn paste(&self) {
        self.terminal.paste_clipboard();
    }

    /// Zoom in by increasing the font scale.
    pub fn zoom_in(&self) {
        let scale = (self.terminal.font_scale() + 0.1).min(5.0);
        self.terminal.set_font_scale(scale);
    }

    /// Zoom out by decreasing the font scale.
    pub fn zoom_out(&self) {
        let scale = (self.terminal.font_scale() - 0.1).max(0.3);
        self.terminal.set_font_scale(scale);
    }

    /// Reset zoom to 1:1.
    pub fn zoom_reset(&self) {
        self.terminal.set_font_scale(1.0);
    }
}

// ---------------------------------------------------------------------------
// Colour helpers
// ---------------------------------------------------------------------------

fn parse_rgba(hex: &str) -> gdk4::RGBA {
    hex.parse().unwrap_or_else(|_| {
        log::warn!("Could not parse colour `{hex}`; using black");
        gdk4::RGBA::new(0.0, 0.0, 0.0, 1.0)
    })
}

fn parse_rgba_with_alpha(hex: &str, alpha: f32) -> gdk4::RGBA {
    let c = parse_rgba(hex);
    gdk4::RGBA::new(c.red(), c.green(), c.blue(), alpha)
}
