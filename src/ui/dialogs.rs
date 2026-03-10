//! Reusable dialog helpers.

#![allow(dead_code)]

use gtk4::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;

/// Show a simple "About MyTerminal" dialog.
pub fn show_about_dialog(parent: &impl IsA<gtk4::Window>) {
    let dialog = adw::AboutWindow::builder()
        .transient_for(parent)
        .modal(true)
        .application_name("MyTerminal")
        .version(env!("CARGO_PKG_VERSION"))
        .developer_name("MyTerminal Contributors")
        .license_type(gtk4::License::Gpl30)
        .website("https://github.com/m4rcel-lol/material-you-terminal-for-arch-linux")
        .issue_url("https://github.com/m4rcel-lol/material-you-terminal-for-arch-linux/issues")
        .comments("A modern, Material You terminal emulator for Linux.")
        .application_icon("utilities-terminal")
        .build();

    dialog.set_developers(&["MyTerminal Contributors"]);
    dialog.present();
}

/// Display a brief toast on `overlay` with `message`.
pub fn show_toast(overlay: &adw::ToastOverlay, message: &str) {
    let toast = adw::Toast::builder()
        .title(message)
        .timeout(3)
        .build();
    overlay.add_toast(toast);
}

/// Show a confirmation dialog and call `on_confirm` if the user accepts.
pub fn confirm_dialog<F: Fn() + 'static>(
    parent: &impl IsA<gtk4::Widget>,
    heading: &str,
    body: &str,
    confirm_label: &str,
    on_confirm: F,
) {
    let dialog = adw::AlertDialog::builder()
        .heading(heading)
        .body(body)
        .build();

    dialog.add_response("cancel", "Cancel");
    dialog.add_response("confirm", confirm_label);
    dialog.set_response_appearance("confirm", adw::ResponseAppearance::Destructive);
    dialog.set_default_response(Some("cancel"));
    dialog.set_close_response("cancel");

    dialog.connect_response(None, move |_, response| {
        if response == "confirm" {
            on_confirm();
        }
    });

    dialog.present(Some(parent));
}

/// Show a "Rename Tab" dialog and call `on_rename` with the new name.
pub fn rename_tab_dialog<F: Fn(String) + 'static>(
    parent: &impl IsA<gtk4::Widget>,
    current_name: &str,
    on_rename: F,
) {
    let dialog = adw::AlertDialog::builder()
        .heading("Rename Tab")
        .build();

    dialog.add_response("cancel", "Cancel");
    dialog.add_response("rename", "Rename");
    dialog.set_default_response(Some("rename"));
    dialog.set_close_response("cancel");

    let entry = gtk4::Entry::builder()
        .text(current_name)
        .activates_default(true)
        .margin_top(6)
        .margin_bottom(6)
        .build();

    dialog.set_extra_child(Some(&entry));

    let entry_clone = entry.clone();
    dialog.connect_response(None, move |_, response| {
        if response == "rename" {
            let new_name = entry_clone.text().to_string();
            if !new_name.is_empty() {
                on_rename(new_name);
            }
        }
    });

    dialog.present(Some(parent));
}

/// Build and return a search bar wired to the given terminal.
pub fn build_search_bar(terminal: &vte4::Terminal) -> gtk4::SearchBar {
    let search_bar = gtk4::SearchBar::new();
    search_bar.set_show_close_button(true);

    let search_entry = gtk4::SearchEntry::new();
    search_bar.set_child(Some(&search_entry));
    search_bar.connect_entry(&search_entry);

    let term_clone = terminal.clone();
    search_entry.connect_search_changed(move |entry| {
        let query = entry.text();
        if query.is_empty() {
            vte4::prelude::TerminalExt::search_set_regex(&term_clone, None, 0);
        } else if let Ok(regex) = vte4::Regex::for_search(&query, 0) {
            vte4::prelude::TerminalExt::search_set_regex(&term_clone, Some(&regex), 0);
            vte4::prelude::TerminalExt::search_find_next(&term_clone);
        }
    });

    let term_clone2 = terminal.clone();
    search_entry.connect_next_match(move |_| {
        vte4::prelude::TerminalExt::search_find_next(&term_clone2);
    });

    let term_clone3 = terminal.clone();
    search_entry.connect_previous_match(move |_| {
        vte4::prelude::TerminalExt::search_find_previous(&term_clone3);
    });

    search_bar
}
