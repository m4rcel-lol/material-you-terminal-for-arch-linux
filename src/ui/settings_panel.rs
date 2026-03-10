//! Settings panel (`adw::PreferencesWindow`).
//!
//! Presents all user-configurable options grouped into pages:
//!  1. Appearance  (theme, font, transparency, cursor)
//!  2. Behaviour   (shell, scrollback, bell)
//!  3. Keybindings (read-only list for now)
//!
//! Changes are applied immediately *and* persisted to disk when the window
//! is closed.

use crate::settings::{CursorShape, Settings};
use crate::themes::ThemeManager;
use gtk4::prelude::*;
use libadwaita::prelude::*;
use libadwaita as adw;
use std::cell::RefCell;
use std::rc::Rc;

/// Callback type that is fired whenever a setting that requires a full UI
/// refresh has changed (theme switch, font change, etc.).
pub type RefreshCallback = Box<dyn Fn()>;

/// Build and return a fully populated preferences window.
pub fn build_settings_window(
    parent: &impl IsA<gtk4::Window>,
    settings: Rc<RefCell<Settings>>,
    theme_manager: Rc<RefCell<ThemeManager>>,
    on_refresh: RefreshCallback,
) -> adw::PreferencesWindow {
    let on_refresh = Rc::new(on_refresh);

    let prefs = adw::PreferencesWindow::builder()
        .title("MyTerminal Settings")
        .transient_for(parent)
        .modal(false)
        .search_enabled(true)
        .build();

    prefs.add(&build_appearance_page(
        Rc::clone(&settings),
        Rc::clone(&theme_manager),
        Rc::clone(&on_refresh),
    ));
    prefs.add(&build_behaviour_page(
        Rc::clone(&settings),
        Rc::clone(&on_refresh),
    ));
    prefs.add(&build_keybindings_page(&settings.borrow()));

    // Persist settings when the window is closed.
    let settings_clone = Rc::clone(&settings);
    prefs.connect_close_request(move |_| {
        if let Err(e) = settings_clone.borrow().save() {
            log::error!("Failed to save settings: {e}");
        }
        glib::Propagation::Proceed
    });

    prefs
}

// ---------------------------------------------------------------------------
// Appearance page
// ---------------------------------------------------------------------------

fn build_appearance_page(
    settings: Rc<RefCell<Settings>>,
    theme_manager: Rc<RefCell<ThemeManager>>,
    on_refresh: Rc<RefreshCallback>,
) -> adw::PreferencesPage {
    let page = adw::PreferencesPage::builder()
        .title("Appearance")
        .icon_name("preferences-desktop-appearance-symbolic")
        .build();

    // ---- Theme group ----
    let theme_group = adw::PreferencesGroup::builder()
        .title("Theme")
        .description("Select a colour theme for the terminal.")
        .build();

    let theme_names: Vec<(String, String)> = theme_manager
        .borrow()
        .theme_names()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    let theme_strings: Vec<&str> = theme_names.iter().map(|(_, v)| v.as_str()).collect();
    let theme_keys: Vec<String> = theme_names.iter().map(|(k, _)| k.clone()).collect();

    let current_idx = theme_keys
        .iter()
        .position(|k| k == theme_manager.borrow().active_key())
        .unwrap_or(0) as u32;

    let theme_model = gtk4::StringList::new(&theme_strings);
    let theme_row = adw::ComboRow::builder()
        .title("Theme")
        .model(&theme_model)
        .selected(current_idx)
        .build();

    {
        let settings = Rc::clone(&settings);
        let theme_manager = Rc::clone(&theme_manager);
        let on_refresh = Rc::clone(&on_refresh);
        let theme_keys = theme_keys.clone();
        theme_row.connect_selected_notify(move |row| {
            let idx = row.selected() as usize;
            if let Some(key) = theme_keys.get(idx) {
                theme_manager.borrow_mut().set_active(key);
                settings.borrow_mut().theme = key.clone();
                (on_refresh)();
            }
        });
    }
    theme_group.add(&theme_row);
    page.add(&theme_group);

    // ---- Font group ----
    let font_group = adw::PreferencesGroup::builder()
        .title("Font")
        .build();

    let font_row = adw::EntryRow::builder()
        .title("Font Family")
        .text(&settings.borrow().font_family)
        .build();
    {
        let settings = Rc::clone(&settings);
        let on_refresh = Rc::clone(&on_refresh);
        font_row.connect_changed(move |entry| {
            let text = entry.text().to_string();
            if !text.is_empty() {
                settings.borrow_mut().font_family = text;
                (on_refresh)();
            }
        });
    }
    font_group.add(&font_row);

    let font_size_adj = gtk4::Adjustment::new(
        settings.borrow().font_size,
        6.0, 72.0, 0.5, 1.0, 0.0,
    );
    let font_size_row = adw::SpinRow::builder()
        .title("Font Size")
        .adjustment(&font_size_adj)
        .digits(1)
        .build();
    {
        let settings = Rc::clone(&settings);
        let on_refresh = Rc::clone(&on_refresh);
        font_size_row.connect_value_notify(move |row| {
            settings.borrow_mut().font_size = row.value();
            (on_refresh)();
        });
    }
    font_group.add(&font_size_row);

    let ligatures_row = adw::SwitchRow::builder()
        .title("Font Ligatures")
        .subtitle("Enable ligatures (requires a ligature font)")
        .active(settings.borrow().font_ligatures)
        .build();
    {
        let settings = Rc::clone(&settings);
        ligatures_row.connect_active_notify(move |row| {
            settings.borrow_mut().font_ligatures = row.is_active();
        });
    }
    font_group.add(&ligatures_row);
    page.add(&font_group);

    // ---- Display group ----
    let display_group = adw::PreferencesGroup::builder()
        .title("Display")
        .build();

    let opacity_adj = gtk4::Adjustment::new(
        settings.borrow().transparency * 100.0,
        10.0, 100.0, 1.0, 5.0, 0.0,
    );
    let opacity_row = adw::SpinRow::builder()
        .title("Background Opacity")
        .subtitle("Percentage of terminal opacity (100 = fully opaque)")
        .adjustment(&opacity_adj)
        .digits(0)
        .build();
    {
        let settings = Rc::clone(&settings);
        let on_refresh = Rc::clone(&on_refresh);
        opacity_row.connect_value_notify(move |row| {
            settings.borrow_mut().transparency = row.value() / 100.0;
            (on_refresh)();
        });
    }
    display_group.add(&opacity_row);

    let padding_adj = gtk4::Adjustment::new(
        settings.borrow().padding as f64,
        0.0, 64.0, 1.0, 4.0, 0.0,
    );
    let padding_row = adw::SpinRow::builder()
        .title("Terminal Padding")
        .subtitle("Inner padding in pixels")
        .adjustment(&padding_adj)
        .digits(0)
        .build();
    {
        let settings = Rc::clone(&settings);
        let on_refresh = Rc::clone(&on_refresh);
        padding_row.connect_value_notify(move |row| {
            settings.borrow_mut().padding = row.value() as i32;
            (on_refresh)();
        });
    }
    display_group.add(&padding_row);
    page.add(&display_group);

    // ---- Cursor group ----
    let cursor_group = adw::PreferencesGroup::builder()
        .title("Cursor")
        .build();

    let cursor_shapes = ["Block", "Underline", "I-Beam"];
    let current_cursor = match settings.borrow().cursor_shape {
        CursorShape::Block => 0u32,
        CursorShape::Underline => 1u32,
        CursorShape::IBeam => 2u32,
    };
    let cursor_model = gtk4::StringList::new(&cursor_shapes);
    let cursor_row = adw::ComboRow::builder()
        .title("Cursor Style")
        .model(&cursor_model)
        .selected(current_cursor)
        .build();
    {
        let settings = Rc::clone(&settings);
        let on_refresh = Rc::clone(&on_refresh);
        cursor_row.connect_selected_notify(move |row| {
            let shape = match row.selected() {
                0 => CursorShape::Block,
                1 => CursorShape::Underline,
                _ => CursorShape::IBeam,
            };
            settings.borrow_mut().cursor_shape = shape;
            (on_refresh)();
        });
    }
    cursor_group.add(&cursor_row);

    let cursor_blink_row = adw::SwitchRow::builder()
        .title("Cursor Blink")
        .active(settings.borrow().cursor_blink)
        .build();
    {
        let settings = Rc::clone(&settings);
        let on_refresh = Rc::clone(&on_refresh);
        cursor_blink_row.connect_active_notify(move |row| {
            settings.borrow_mut().cursor_blink = row.is_active();
            (on_refresh)();
        });
    }
    cursor_group.add(&cursor_blink_row);
    page.add(&cursor_group);

    page
}

// ---------------------------------------------------------------------------
// Behaviour page
// ---------------------------------------------------------------------------

fn build_behaviour_page(
    settings: Rc<RefCell<Settings>>,
    _on_refresh: Rc<RefreshCallback>,
) -> adw::PreferencesPage {
    let page = adw::PreferencesPage::builder()
        .title("Behaviour")
        .icon_name("preferences-system-symbolic")
        .build();

    // ---- Shell group ----
    let shell_group = adw::PreferencesGroup::builder()
        .title("Shell")
        .build();

    let shell_value = settings
        .borrow()
        .shell
        .clone()
        .unwrap_or_default();
    let shell_row = adw::EntryRow::builder()
        .title("Shell")
        .text(&shell_value)
        .input_purpose(gtk4::InputPurpose::Terminal)
        .build();
    shell_row
        .set_tooltip_text(Some("Leave empty to use $SHELL from the environment"));
    {
        let settings = Rc::clone(&settings);
        shell_row.connect_changed(move |entry| {
            let text = entry.text().to_string();
            settings.borrow_mut().shell = if text.is_empty() { None } else { Some(text) };
        });
    }
    shell_group.add(&shell_row);

    let startup_dir_value = settings
        .borrow()
        .startup_directory
        .clone()
        .unwrap_or_default();
    let startup_row = adw::EntryRow::builder()
        .title("Startup Directory")
        .text(&startup_dir_value)
        .build();
    startup_row.set_tooltip_text(Some("Leave empty to use the home directory"));
    {
        let settings = Rc::clone(&settings);
        startup_row.connect_changed(move |entry| {
            let text = entry.text().to_string();
            settings.borrow_mut().startup_directory =
                if text.is_empty() { None } else { Some(text) };
        });
    }
    shell_group.add(&startup_row);
    page.add(&shell_group);

    // ---- Terminal group ----
    let terminal_group = adw::PreferencesGroup::builder()
        .title("Terminal")
        .build();

    let scrollback_adj = gtk4::Adjustment::new(
        settings.borrow().scrollback_lines as f64,
        -1.0, 1_000_000.0, 100.0, 1000.0, 0.0,
    );
    let scrollback_row = adw::SpinRow::builder()
        .title("Scrollback Lines")
        .subtitle("-1 for unlimited")
        .adjustment(&scrollback_adj)
        .digits(0)
        .build();
    {
        let settings = Rc::clone(&settings);
        scrollback_row.connect_value_notify(move |row| {
            settings.borrow_mut().scrollback_lines = row.value() as i64;
        });
    }
    terminal_group.add(&scrollback_row);

    let bell_row = adw::SwitchRow::builder()
        .title("Terminal Bell")
        .subtitle("Play a sound on terminal bell (BEL)")
        .active(settings.borrow().bell)
        .build();
    {
        let settings = Rc::clone(&settings);
        bell_row.connect_active_notify(move |row| {
            settings.borrow_mut().bell = row.is_active();
        });
    }
    terminal_group.add(&bell_row);
    page.add(&terminal_group);

    page
}

// ---------------------------------------------------------------------------
// Keybindings page
// ---------------------------------------------------------------------------

fn build_keybindings_page(settings: &Settings) -> adw::PreferencesPage {
    let page = adw::PreferencesPage::builder()
        .title("Keybindings")
        .icon_name("input-keyboard-symbolic")
        .build();

    let group = adw::PreferencesGroup::builder()
        .title("Keyboard Shortcuts")
        .description("Current keyboard shortcuts (customisable in config.json)")
        .build();

    let kb = &settings.keybindings;
    let shortcuts: &[(&str, &str)] = &[
        ("New Tab", &kb.new_tab),
        ("Close Tab", &kb.close_tab),
        ("Next Tab", &kb.next_tab),
        ("Previous Tab", &kb.prev_tab),
        ("Search", &kb.search),
        ("Command Palette", &kb.command_palette),
        ("Zoom In", &kb.zoom_in),
        ("Zoom Out", &kb.zoom_out),
        ("Reset Zoom", &kb.zoom_reset),
        ("Split Horizontal", &kb.split_horizontal),
        ("Split Vertical", &kb.split_vertical),
        ("Copy", &kb.copy),
        ("Paste", &kb.paste),
    ];

    for (label, accel) in shortcuts {
        let row = adw::ActionRow::builder()
            .title(*label)
            .build();
        let badge = gtk4::ShortcutLabel::new(accel);
        badge.set_valign(gtk4::Align::Center);
        row.add_suffix(&badge);
        group.add(&row);
    }

    page.add(&group);
    page
}
