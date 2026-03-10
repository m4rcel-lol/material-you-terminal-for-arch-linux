//! Custom header bar for the MyTerminal window.
//!
//! Builds a `adw::HeaderBar` containing:
//!  – new-tab button (left side)
//!  – application menu button (right side, hamburger)
//!  – search toggle (right side)

use gtk4::prelude::*;
use libadwaita as adw;

/// Build the application header bar and return it together with action
/// buttons that the caller needs to wire up.
pub struct HeaderBar {
    /// The underlying Adwaita header bar widget.
    pub bar: adw::HeaderBar,
    /// Button to open a new tab (kept for external use / theming).
    #[allow(dead_code)]
    pub new_tab_button: gtk4::Button,
    /// Button to open the search bar.
    pub search_button: gtk4::ToggleButton,
    /// Primary / hamburger menu button (kept for external use / theming).
    #[allow(dead_code)]
    pub menu_button: gtk4::MenuButton,
}

impl HeaderBar {
    /// Construct the header bar and all its child widgets.
    pub fn new() -> Self {
        let bar = adw::HeaderBar::new();
        bar.set_show_end_title_buttons(true);
        bar.add_css_class("myterminal-headerbar");

        // ----- Left-side widgets -----

        let new_tab_button = gtk4::Button::builder()
            .icon_name("tab-new-symbolic")
            .tooltip_text("New Tab (Ctrl+Shift+T)")
            .action_name("win.new-tab")
            .build();
        new_tab_button.add_css_class("flat");
        bar.pack_start(&new_tab_button);

        // ----- Right-side widgets -----

        let search_button = gtk4::ToggleButton::builder()
            .icon_name("edit-find-symbolic")
            .tooltip_text("Search (Ctrl+Shift+F)")
            .build();
        search_button.add_css_class("flat");
        bar.pack_end(&search_button);

        let menu = Self::build_primary_menu();
        let menu_button = gtk4::MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .tooltip_text("Main Menu")
            .menu_model(&menu)
            .build();
        menu_button.add_css_class("flat");
        bar.pack_end(&menu_button);

        HeaderBar {
            bar,
            new_tab_button,
            search_button,
            menu_button,
        }
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    fn build_primary_menu() -> gio::Menu {
        let menu = gio::Menu::new();

        let terminal_section = gio::Menu::new();
        terminal_section.append(Some("New Tab"), Some("win.new-tab"));
        terminal_section.append(Some("Close Tab"), Some("win.close-tab"));
        terminal_section.append(Some("Duplicate Tab"), Some("win.duplicate-tab"));
        terminal_section.append(Some("Split Horizontal"), Some("win.split-horizontal"));
        terminal_section.append(Some("Split Vertical"), Some("win.split-vertical"));
        menu.append_section(Some("Terminal"), &terminal_section);

        let view_section = gio::Menu::new();
        view_section.append(Some("Zoom In"), Some("win.zoom-in"));
        view_section.append(Some("Zoom Out"), Some("win.zoom-out"));
        view_section.append(Some("Reset Zoom"), Some("win.zoom-reset"));
        menu.append_section(Some("View"), &view_section);

        let app_section = gio::Menu::new();
        app_section.append(Some("Command Palette"), Some("win.command-palette"));
        app_section.append(Some("Preferences"), Some("win.open-settings"));
        app_section.append(Some("Keyboard Shortcuts"), Some("win.show-shortcuts"));
        app_section.append(Some("About MyTerminal"), Some("win.show-about"));
        menu.append_section(Some("Application"), &app_section);

        menu
    }
}

impl Default for HeaderBar {
    fn default() -> Self {
        Self::new()
    }
}
