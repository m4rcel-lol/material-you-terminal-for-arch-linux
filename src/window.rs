//! Main application window.
//!
//! Assembles the full UI from sub-components:
//!
//! ```text
//! AdwApplicationWindow
//!  └── ToastOverlay
//!       └── ToolbarView
//!            ├── [top bar] HeaderBar   (adw)
//!            ├── [top bar] TabBar      (adw)
//!            └── [content] Stack
//!                 ├── TabView          (adw) ← terminal pages
//!                 └── SearchBar        (gtk)
//! ```
//!
//! All window-level GActions are registered here so that keyboard shortcuts
//! registered in `app.rs` work out of the box.

use crate::commands::build_command_palette;
use crate::settings::Settings;
use crate::tabs::TabManager;
use crate::themes::ThemeManager;
use crate::ui::dialogs::{show_about_dialog, show_toast};
use crate::ui::headerbar::HeaderBar as MyHeaderBar;
use crate::ui::settings_panel::build_settings_window;
use gtk4::prelude::*;
use libadwaita::prelude::*;
use libadwaita as adw;
use std::cell::RefCell;
use std::rc::Rc;

// ---------------------------------------------------------------------------
// CSS
// ---------------------------------------------------------------------------

const APP_CSS: &str = r#"
/* ── Window ──────────────────────────────────────────────────────────────── */
window.myterminal {
    background-color: transparent;
}

/* ── Header bar ──────────────────────────────────────────────────────────── */
.myterminal-headerbar {
    background-color: @headerbar_bg_color;
    border-bottom: 1px solid alpha(@border_color, 0.5);
    min-height: 48px;
    box-shadow: 0 2px 8px alpha(black, 0.1);
}

.myterminal-headerbar button {
    border-radius: 8px;
    transition: all 200ms cubic-bezier(0.4, 0.0, 0.2, 1);
}

.myterminal-headerbar button:hover {
    background-color: alpha(@accent_color, 0.1);
}

/* ── Tab bar ─────────────────────────────────────────────────────────────── */
tabbar {
    background-color: @headerbar_bg_color;
    padding: 4px 8px 0 8px;
}

tabbar tab {
    border-radius: 10px 10px 0 0;
    min-height: 38px;
    padding: 0 16px;
    margin: 0 2px;
    transition: all 200ms cubic-bezier(0.4, 0.0, 0.2, 1);
    background-color: transparent;
}

tabbar tab:hover {
    background-color: alpha(@accent_color, 0.08);
}

tabbar tab:checked {
    background-color: @view_bg_color;
    box-shadow: 0 -2px 8px alpha(black, 0.08);
    font-weight: 500;
}

tabbar .start-action,
tabbar .end-action {
    padding: 0 8px;
}

/* ── Tab close button styling ─────────────────────────────────────────────── */
tabbar tab button.close-button {
    border-radius: 6px;
    min-width: 20px;
    min-height: 20px;
    padding: 0;
    margin: 0 4px;
    opacity: 0.7;
    transition: all 150ms cubic-bezier(0.4, 0.0, 0.2, 1);
}

tabbar tab button.close-button:hover {
    opacity: 1;
    background-color: alpha(@error_color, 0.15);
}

/* ── Search bar ──────────────────────────────────────────────────────────── */
searchbar {
    background-color: @headerbar_bg_color;
    border-bottom: 1px solid alpha(@border_color, 0.3);
    padding: 8px;
}

searchbar entry {
    border-radius: 10px;
    min-height: 36px;
    box-shadow: 0 2px 6px alpha(black, 0.08);
}

/* ── Terminal view ───────────────────────────────────────────────────────── */
.terminal-view {
    border-radius: 0;
    background-color: @view_bg_color;
}

/* ── Command palette ─────────────────────────────────────────────────────── */
.command-palette {
    border-radius: 16px;
    box-shadow: 0 12px 48px alpha(black, 0.5);
}

.command-palette-box {
    background-color: @dialog_bg_color;
    border-radius: 16px;
    border: 1px solid alpha(@border_color, 0.2);
}

.command-palette entry {
    border-radius: 12px;
    min-height: 40px;
    font-size: 14px;
}

.command-palette listview row {
    border-radius: 8px;
    margin: 2px 6px;
    padding: 10px 12px;
    transition: all 150ms cubic-bezier(0.4, 0.0, 0.2, 1);
}

.command-palette listview row:hover {
    background-color: alpha(@accent_color, 0.12);
}

.command-palette listview row:selected {
    background-color: alpha(@accent_color, 0.2);
}

/* ── Toast ───────────────────────────────────────────────────────────────── */
toast {
    border-radius: 28px;
    box-shadow: 0 4px 16px alpha(black, 0.2);
}

/* ── Scrollbars ──────────────────────────────────────────────────────────── */
scrollbar {
    background-color: transparent;
}

scrollbar slider {
    border-radius: 8px;
    min-width: 8px;
    min-height: 8px;
    background-color: alpha(@window_fg_color, 0.3);
    transition: all 200ms cubic-bezier(0.4, 0.0, 0.2, 1);
}

scrollbar slider:hover {
    background-color: alpha(@window_fg_color, 0.5);
    min-width: 10px;
}

scrollbar slider:active {
    background-color: alpha(@window_fg_color, 0.7);
}

/* ── Preferences window ──────────────────────────────────────────────────── */
preferencespage {
    background-color: @view_bg_color;
}

preferencesgroup {
    border-radius: 12px;
    background-color: @card_bg_color;
}

actionrow, expander row {
    border-radius: 8px;
    transition: background-color 150ms cubic-bezier(0.4, 0.0, 0.2, 1);
}

actionrow:hover, expander row:hover {
    background-color: alpha(@accent_color, 0.08);
}

/* ── Buttons ─────────────────────────────────────────────────────────────── */
button {
    border-radius: 8px;
    transition: all 200ms cubic-bezier(0.4, 0.0, 0.2, 1);
}

button.suggested-action {
    background: linear-gradient(to bottom, @accent_bg_color, alpha(@accent_bg_color, 0.95));
    box-shadow: 0 2px 8px alpha(@accent_bg_color, 0.3);
}

button.suggested-action:hover {
    box-shadow: 0 4px 12px alpha(@accent_bg_color, 0.4);
}

button.destructive-action {
    background: linear-gradient(to bottom, @error_bg_color, alpha(@error_bg_color, 0.95));
    box-shadow: 0 2px 8px alpha(@error_bg_color, 0.3);
}

button.destructive-action:hover {
    box-shadow: 0 4px 12px alpha(@error_bg_color, 0.4);
}

/* ── Menu items ──────────────────────────────────────────────────────────── */
menuitem {
    border-radius: 6px;
    margin: 2px 4px;
    padding: 8px 12px;
    transition: all 150ms cubic-bezier(0.4, 0.0, 0.2, 1);
}

menuitem:hover {
    background-color: alpha(@accent_color, 0.12);
}

/* ── Window decorations ──────────────────────────────────────────────────── */
window {
    box-shadow: 0 8px 32px alpha(black, 0.3);
}

window.maximized {
    box-shadow: none;
}
"#;

// ---------------------------------------------------------------------------
// MyTerminalWindow
// ---------------------------------------------------------------------------

/// Top-level window for a single MyTerminal instance.
pub struct MyTerminalWindow {
    window: adw::ApplicationWindow,
}

impl MyTerminalWindow {
    /// Build the complete window UI, register GActions, and return the wrapper.
    pub fn new(
        app: &adw::Application,
        settings: Rc<RefCell<Settings>>,
        theme_manager: Rc<RefCell<ThemeManager>>,
    ) -> Self {
        // Apply global CSS.
        Self::apply_css();

        // ── Outer window ──────────────────────────────────────────────────
        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("MyTerminal")
            .default_width(960)
            .default_height(640)
            .css_classes(["myterminal"])
            .build();

        // ── Toast overlay (wraps everything so toasts appear on top) ──────
        let toast_overlay = adw::ToastOverlay::new();

        // ── Tab view + bar ─────────────────────────────────────────────────
        let tab_view = adw::TabView::new();
        let tab_bar = adw::TabBar::builder()
            .view(&tab_view)
            .autohide(false)
            .expand_tabs(false)
            .build();

        // ── Tab manager ───────────────────────────────────────────────────
        let tab_manager = Rc::new(RefCell::new(TabManager::new(
            tab_view.clone(),
            Rc::clone(&settings),
        )));

        // Open the first tab immediately.
        {
            let theme = theme_manager.borrow().active().clone();
            tab_manager.borrow_mut().new_tab(&theme);
        }

        // ── Search bar ────────────────────────────────────────────────────
        let search_bar = gtk4::SearchBar::new();
        search_bar.set_show_close_button(true);
        let search_entry = gtk4::SearchEntry::new();
        search_bar.set_child(Some(&search_entry));
        search_bar.connect_entry(&search_entry);

        // ── Content box (tab_bar + search_bar + tab_view) ─────────────────
        let content_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
        content_box.append(&tab_bar);
        content_box.append(&search_bar);
        content_box.append(&tab_view);
        tab_view.set_vexpand(true);

        // ── Toolbar view (holds header + content) ─────────────────────────
        let toolbar_view = adw::ToolbarView::new();
        let header_bar_wrapper = MyHeaderBar::new();

        // Wire search toggle button to reveal the search bar.
        {
            let search_bar_clone = search_bar.clone();
            header_bar_wrapper
                .search_button
                .connect_toggled(move |btn| {
                    search_bar_clone.set_search_mode(btn.is_active());
                });
        }

        toolbar_view.add_top_bar(&header_bar_wrapper.bar);
        toolbar_view.set_content(Some(&content_box));

        toast_overlay.set_child(Some(&toolbar_view));
        window.set_content(Some(&toast_overlay));

        // ── Register GActions ─────────────────────────────────────────────
        Self::register_actions(
            &window,
            &tab_manager,
            &settings,
            &theme_manager,
            &toast_overlay,
            &search_bar,
            &header_bar_wrapper.search_button,
        );

        // ── Auto-close window when the last tab is removed ────────────────
        {
            let window_clone = window.clone();
            tab_view.connect_n_pages_notify(move |tv| {
                if tv.n_pages() == 0 {
                    window_clone.close();
                }
            });
        }

        MyTerminalWindow { window }
    }

    /// Present (show and raise) the window.
    pub fn present(&self) {
        self.window.present();
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    fn apply_css() {
        let provider = gtk4::CssProvider::new();
        provider.load_from_string(APP_CSS);
        if let Some(display) = gdk4::Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    /// Register all `win.*` GActions.
    fn register_actions(
        window: &adw::ApplicationWindow,
        tab_manager: &Rc<RefCell<TabManager>>,
        settings: &Rc<RefCell<Settings>>,
        theme_manager: &Rc<RefCell<ThemeManager>>,
        toast_overlay: &adw::ToastOverlay,
        search_bar: &gtk4::SearchBar,
        search_button: &gtk4::ToggleButton,
    ) {
        // ── new-tab ────────────────────────────────────────────────────────
        {
            let act = gio::SimpleAction::new("new-tab", None);
            let tab_mgr = Rc::clone(tab_manager);
            let theme_mgr = Rc::clone(theme_manager);
            act.connect_activate(move |_, _| {
                let theme = theme_mgr.borrow().active().clone();
                tab_mgr.borrow_mut().new_tab(&theme);
            });
            window.add_action(&act);
        }

        // ── close-tab ─────────────────────────────────────────────────────
        {
            let act = gio::SimpleAction::new("close-tab", None);
            let tab_mgr = Rc::clone(tab_manager);
            act.connect_activate(move |_, _| {
                tab_mgr.borrow().close_current_tab();
            });
            window.add_action(&act);
        }

        // ── next-tab ──────────────────────────────────────────────────────
        {
            let act = gio::SimpleAction::new("next-tab", None);
            let tab_mgr = Rc::clone(tab_manager);
            act.connect_activate(move |_, _| {
                tab_mgr.borrow().next_tab();
            });
            window.add_action(&act);
        }

        // ── prev-tab ──────────────────────────────────────────────────────
        {
            let act = gio::SimpleAction::new("prev-tab", None);
            let tab_mgr = Rc::clone(tab_manager);
            act.connect_activate(move |_, _| {
                tab_mgr.borrow().prev_tab();
            });
            window.add_action(&act);
        }

        // ── duplicate-tab ─────────────────────────────────────────────────
        {
            let act = gio::SimpleAction::new("duplicate-tab", None);
            let tab_mgr = Rc::clone(tab_manager);
            let theme_mgr = Rc::clone(theme_manager);
            act.connect_activate(move |_, _| {
                let theme = theme_mgr.borrow().active().clone();
                tab_mgr.borrow_mut().duplicate_current_tab(&theme);
            });
            window.add_action(&act);
        }

        // ── copy ──────────────────────────────────────────────────────────
        {
            let act = gio::SimpleAction::new("copy", None);
            let tab_mgr = Rc::clone(tab_manager);
            let overlay = toast_overlay.clone();
            act.connect_activate(move |_, _| {
                tab_mgr.borrow().copy_current();
                show_toast(&overlay, "Copied to clipboard");
            });
            window.add_action(&act);
        }

        // ── paste ─────────────────────────────────────────────────────────
        {
            let act = gio::SimpleAction::new("paste", None);
            let tab_mgr = Rc::clone(tab_manager);
            act.connect_activate(move |_, _| {
                tab_mgr.borrow().paste_current();
            });
            window.add_action(&act);
        }

        // ── zoom-in ───────────────────────────────────────────────────────
        {
            let act = gio::SimpleAction::new("zoom-in", None);
            let tab_mgr = Rc::clone(tab_manager);
            act.connect_activate(move |_, _| {
                tab_mgr.borrow().zoom_in_current();
            });
            window.add_action(&act);
        }

        // ── zoom-out ──────────────────────────────────────────────────────
        {
            let act = gio::SimpleAction::new("zoom-out", None);
            let tab_mgr = Rc::clone(tab_manager);
            act.connect_activate(move |_, _| {
                tab_mgr.borrow().zoom_out_current();
            });
            window.add_action(&act);
        }

        // ── zoom-reset ────────────────────────────────────────────────────
        {
            let act = gio::SimpleAction::new("zoom-reset", None);
            let tab_mgr = Rc::clone(tab_manager);
            act.connect_activate(move |_, _| {
                tab_mgr.borrow().zoom_reset_current();
            });
            window.add_action(&act);
        }

        // ── search ────────────────────────────────────────────────────────
        {
            let act = gio::SimpleAction::new("search", None);
            let search_bar_clone = search_bar.clone();
            let search_btn_clone = search_button.clone();
            act.connect_activate(move |_, _| {
                let new_state = !search_bar_clone.is_search_mode();
                search_bar_clone.set_search_mode(new_state);
                search_btn_clone.set_active(new_state);
            });
            window.add_action(&act);
        }

        // ── split-horizontal (placeholder — creates new tab for now) ──────
        {
            let act = gio::SimpleAction::new("split-horizontal", None);
            let tab_mgr = Rc::clone(tab_manager);
            let theme_mgr = Rc::clone(theme_manager);
            act.connect_activate(move |_, _| {
                let theme = theme_mgr.borrow().active().clone();
                tab_mgr.borrow_mut().new_tab(&theme);
            });
            window.add_action(&act);
        }

        // ── split-vertical (placeholder — creates new tab for now) ────────
        {
            let act = gio::SimpleAction::new("split-vertical", None);
            let tab_mgr = Rc::clone(tab_manager);
            let theme_mgr = Rc::clone(theme_manager);
            act.connect_activate(move |_, _| {
                let theme = theme_mgr.borrow().active().clone();
                tab_mgr.borrow_mut().new_tab(&theme);
            });
            window.add_action(&act);
        }

        // ── clear-terminal ────────────────────────────────────────────────
        {
            let act = gio::SimpleAction::new("clear-terminal", None);
            let tab_mgr = Rc::clone(tab_manager);
            act.connect_activate(move |_, _| {
                tab_mgr.borrow().clear_current();
            });
            window.add_action(&act);
        }

        // ── open-settings ─────────────────────────────────────────────────
        {
            let act = gio::SimpleAction::new("open-settings", None);
            let window_clone = window.clone();
            let settings_clone = Rc::clone(settings);
            let theme_mgr_clone = Rc::clone(theme_manager);
            let tab_mgr_clone = Rc::clone(tab_manager);
            act.connect_activate(move |_, _| {
                let refresh_tab_mgr = Rc::clone(&tab_mgr_clone);
                let refresh_theme_mgr = Rc::clone(&theme_mgr_clone);
                let prefs = build_settings_window(
                    &window_clone,
                    Rc::clone(&settings_clone),
                    Rc::clone(&theme_mgr_clone),
                    Box::new(move || {
                        let theme = refresh_theme_mgr.borrow().active().clone();
                        refresh_tab_mgr.borrow().apply_theme_to_all(&theme);
                    }),
                );
                prefs.present();
            });
            window.add_action(&act);
        }

        // ── command-palette ───────────────────────────────────────────────
        {
            let act = gio::SimpleAction::new("command-palette", None);
            let window_clone = window.clone();
            act.connect_activate(move |_, _| {
                let parent_clone = window_clone.clone();
                let palette = build_command_palette(&window_clone, move |cmd_id| {
                    let _ = gtk4::prelude::WidgetExt::activate_action(
                        &parent_clone,
                        &format!("win.{cmd_id}"),
                        None,
                    );
                });
                palette.present();
            });
            window.add_action(&act);
        }

        // ── show-about ────────────────────────────────────────────────────
        {
            let act = gio::SimpleAction::new("show-about", None);
            let window_clone = window.clone();
            act.connect_activate(move |_, _| {
                show_about_dialog(&window_clone);
            });
            window.add_action(&act);
        }

        // ── show-shortcuts ────────────────────────────────────────────────
        {
            let act = gio::SimpleAction::new("show-shortcuts", None);
            let window_clone = window.clone();
            act.connect_activate(move |_, _| {
                let sw = gtk4::ShortcutsWindow::builder()
                    .transient_for(&window_clone)
                    .modal(true)
                    .build();
                sw.present();
            });
            window.add_action(&act);
        }
    }
}
