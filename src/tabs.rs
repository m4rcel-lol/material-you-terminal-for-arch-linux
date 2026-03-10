//! Tab management.
//!
//! `TabManager` owns the `adw::TabView` and handles all tab-related actions.
//! Terminal instances are accessed by traversing the tab page's widget tree
//! rather than maintaining a separate parallel Vec, which avoids sync issues.

use crate::settings::Settings;
use crate::terminal::TerminalWidget;
use crate::themes::Theme;
use gtk4::prelude::*;
use libadwaita as adw;
use std::cell::RefCell;
use std::rc::Rc;
use vte4::prelude::*;

// ---------------------------------------------------------------------------
// TabManager
// ---------------------------------------------------------------------------

/// Manages all open tabs inside an `adw::TabView`.
pub struct TabManager {
    tab_view: adw::TabView,
    settings: Rc<RefCell<Settings>>,
    tab_counter: u32,
}

impl TabManager {
    /// Create a manager wrapping the given `TabView`.
    pub fn new(tab_view: adw::TabView, settings: Rc<RefCell<Settings>>) -> Self {
        TabManager {
            tab_view,
            settings,
            tab_counter: 0,
        }
    }

    // -----------------------------------------------------------------------
    // Public actions
    // -----------------------------------------------------------------------

    /// Open a new tab and return its `adw::TabPage`.
    pub fn new_tab(&mut self, theme: &Theme) -> adw::TabPage {
        self.tab_counter += 1;
        let title = format!("Terminal {}", self.tab_counter);

        let settings = self.settings.borrow().clone();
        let term = TerminalWidget::new(&settings);
        term.apply_theme(theme, settings.transparency);
        term.spawn_shell(&settings);

        // Wrap the VTE widget in a ScrolledWindow.
        let scroll = gtk4::ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .vexpand(true)
            .hexpand(true)
            .build();
        scroll.set_child(Some(&term.terminal));

        // Close the tab automatically when the child process exits.
        let tab_view_clone = self.tab_view.clone();
        let scroll_clone = scroll.clone();
        term.terminal.connect_child_exited(move |_terminal, _status| {
            let n = tab_view_clone.n_pages();
            for i in 0..n {
                let page = tab_view_clone.nth_page(i);
                if page.child() == scroll_clone.clone().upcast::<gtk4::Widget>() {
                    tab_view_clone.close_page(&page);
                    return;
                }
            }
        });

        let page = self.tab_view.append(&scroll);
        page.set_title(&title);
        self.tab_view.set_selected_page(&page);
        page
    }

    /// Close the currently selected tab.
    pub fn close_current_tab(&self) {
        if let Some(page) = self.tab_view.selected_page() {
            self.tab_view.close_page(&page);
        }
    }

    /// Switch to the next tab (wraps around).
    pub fn next_tab(&self) {
        let n = self.tab_view.n_pages();
        if n <= 1 {
            return;
        }
        if let Some(page) = self.tab_view.selected_page() {
            let idx = self.tab_view.page_position(&page);
            self.tab_view
                .set_selected_page(&self.tab_view.nth_page((idx + 1) % n));
        }
    }

    /// Switch to the previous tab (wraps around).
    pub fn prev_tab(&self) {
        let n = self.tab_view.n_pages();
        if n <= 1 {
            return;
        }
        if let Some(page) = self.tab_view.selected_page() {
            let idx = self.tab_view.page_position(&page);
            self.tab_view
                .set_selected_page(&self.tab_view.nth_page((idx + n - 1) % n));
        }
    }

    /// Duplicate the currently selected tab (opens a new tab with the same settings).
    pub fn duplicate_current_tab(&mut self, theme: &Theme) {
        self.new_tab(theme);
    }

    /// Re-apply a new theme to all open terminals.
    pub fn apply_theme_to_all(&self, theme: &Theme) {
        let opacity = self.settings.borrow().transparency;
        for terminal in self.all_terminals() {
            terminal.apply_theme(theme, opacity);
        }
    }

    /// Copy from the currently selected terminal.
    pub fn copy_current(&self) {
        if let Some(term) = self.active_terminal() {
            term.copy();
        }
    }

    /// Paste into the currently selected terminal.
    pub fn paste_current(&self) {
        if let Some(term) = self.active_terminal() {
            term.paste();
        }
    }

    /// Zoom in on the current terminal.
    pub fn zoom_in_current(&self) {
        if let Some(term) = self.active_terminal() {
            term.zoom_in();
        }
    }

    /// Zoom out on the current terminal.
    pub fn zoom_out_current(&self) {
        if let Some(term) = self.active_terminal() {
            term.zoom_out();
        }
    }

    /// Reset zoom on the current terminal.
    pub fn zoom_reset_current(&self) {
        if let Some(term) = self.active_terminal() {
            term.zoom_reset();
        }
    }

    /// Clear the current terminal's screen and scrollback.
    pub fn clear_current(&self) {
        if let Some(term) = self.active_terminal() {
            term.terminal.feed(b"\x1b[2J\x1b[H");
        }
    }

    /// Return the underlying `adw::TabView`.
    #[allow(dead_code)]
    pub fn tab_view(&self) -> &adw::TabView {
        &self.tab_view
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    /// Return a `TerminalWidget` wrapping the VTE terminal in the active tab.
    fn active_terminal(&self) -> Option<TerminalWidget> {
        let page = self.tab_view.selected_page()?;
        terminal_from_page(&page)
    }

    /// Return `TerminalWidget` for every open tab.
    fn all_terminals(&self) -> Vec<TerminalWidget> {
        (0..self.tab_view.n_pages())
            .filter_map(|i| {
                let page = self.tab_view.nth_page(i);
                terminal_from_page(&page)
            })
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Widget-tree helpers
// ---------------------------------------------------------------------------

/// Extract the `vte4::Terminal` from a tab page, wrapping it in a
/// `TerminalWidget` for convenient method access.
fn terminal_from_page(page: &adw::TabPage) -> Option<TerminalWidget> {
    let scroll = page.child().downcast::<gtk4::ScrolledWindow>().ok()?;
    let vte = scroll.child()?.downcast::<vte4::Terminal>().ok()?;
    Some(TerminalWidget { terminal: vte })
}
