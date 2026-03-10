//! Command palette (Ctrl+Shift+P).
//!
//! A floating search dialog that lets the user quickly find and execute
//! any application command.  Inspired by VS Code's command palette.

use gtk4::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::*;

/// An individual entry in the command palette.
#[derive(Clone)]
pub struct Command {
    pub id: &'static str,
    pub label: &'static str,
    pub subtitle: &'static str,
    pub icon: &'static str,
}

/// All available commands, displayed in the palette.
pub const COMMANDS: &[Command] = &[
    Command { id: "new-tab",         label: "New Tab",           subtitle: "Ctrl+Shift+T", icon: "tab-new-symbolic" },
    Command { id: "close-tab",       label: "Close Tab",         subtitle: "Ctrl+Shift+W", icon: "window-close-symbolic" },
    Command { id: "next-tab",        label: "Next Tab",          subtitle: "Ctrl+Tab",     icon: "go-next-symbolic" },
    Command { id: "prev-tab",        label: "Previous Tab",      subtitle: "Ctrl+Shift+Tab", icon: "go-previous-symbolic" },
    Command { id: "duplicate-tab",   label: "Duplicate Tab",     subtitle: "",             icon: "edit-copy-symbolic" },
    Command { id: "split-horizontal",  label: "Split Horizontal",  subtitle: "Ctrl+Shift+D", icon: "object-flip-vertical-symbolic" },
    Command { id: "split-vertical",    label: "Split Vertical",    subtitle: "Ctrl+D",       icon: "object-flip-horizontal-symbolic" },
    Command { id: "search",          label: "Search in Terminal",subtitle: "Ctrl+Shift+F", icon: "edit-find-symbolic" },
    Command { id: "zoom-in",         label: "Zoom In",           subtitle: "Ctrl++",       icon: "zoom-in-symbolic" },
    Command { id: "zoom-out",        label: "Zoom Out",          subtitle: "Ctrl+-",       icon: "zoom-out-symbolic" },
    Command { id: "zoom-reset",      label: "Reset Zoom",        subtitle: "Ctrl+0",       icon: "zoom-original-symbolic" },
    Command { id: "open-settings",   label: "Open Settings",     subtitle: "Ctrl+,",       icon: "preferences-system-symbolic" },
    Command { id: "copy",            label: "Copy",              subtitle: "Ctrl+Shift+C", icon: "edit-copy-symbolic" },
    Command { id: "paste",           label: "Paste",             subtitle: "Ctrl+Shift+V", icon: "edit-paste-symbolic" },
    Command { id: "clear-terminal",  label: "Clear Terminal",    subtitle: "",             icon: "edit-clear-all-symbolic" },
];

/// Build and return the command palette window.
///
/// The caller must set the transient parent and present it.
/// `on_activate` is called with the command `id` when the user selects one.
pub fn build_command_palette<F>(parent: &impl IsA<gtk4::Window>, on_activate: F) -> gtk4::Window
where
    F: Fn(&str) + 'static,
{
    let on_activate = std::rc::Rc::new(on_activate);

    // Outer container window — modal, undecorated, centred.
    let palette_window = gtk4::Window::builder()
        .title("Command Palette")
        .transient_for(parent)
        .modal(true)
        .decorated(false)
        .width_request(560)
        .height_request(400)
        .css_classes(["command-palette"])
        .build();

    let outer_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    outer_box.add_css_class("command-palette-box");

    // Search entry at the top.
    let search_entry = gtk4::SearchEntry::builder()
        .placeholder_text("Search commands…")
        .margin_start(12)
        .margin_end(12)
        .margin_top(12)
        .margin_bottom(6)
        .build();

    // Scrollable list below the entry.
    let scroll = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .vexpand(true)
        .margin_bottom(8)
        .build();

    let list_box = gtk4::ListBox::builder()
        .selection_mode(gtk4::SelectionMode::Single)
        .css_classes(["boxed-list"])
        .margin_start(12)
        .margin_end(12)
        .build();

    // Populate list with all commands.
    for cmd in COMMANDS {
        let row = adw::ActionRow::builder()
            .title(cmd.label)
            .subtitle(cmd.subtitle)
            .activatable(true)
            .build();
        row.add_prefix(&gtk4::Image::from_icon_name(cmd.icon));
        row.set_widget_name(cmd.id);
        list_box.append(&row);
    }

    scroll.set_child(Some(&list_box));
    outer_box.append(&search_entry);
    outer_box.append(&scroll);
    palette_window.set_child(Some(&outer_box));

    // Filter list on search text change.
    let list_box_clone = list_box.clone();
    search_entry.connect_search_changed(move |entry| {
        let query = entry.text().to_lowercase();
        let mut child = list_box_clone.first_child();
        while let Some(row) = child {
            let next = row.next_sibling();
            if let Some(action_row) = row.downcast_ref::<adw::ActionRow>() {
                let visible = query.is_empty()
                    || action_row.title().to_lowercase().contains(&query)
                    || action_row
                        .subtitle()
                        .map(|s| s.to_lowercase().contains(&query))
                        .unwrap_or(false);
                action_row.set_visible(visible);
            }
            child = next;
        }
    });

    // Activate command on row selection.
    let on_activate_clone = on_activate.clone();
    let palette_window_clone = palette_window.clone();
    list_box.connect_row_activated(move |_, row| {
        let id = row.widget_name();
        on_activate_clone(id.as_str());
        palette_window_clone.close();
    });

    // Close on Escape.
    let controller = gtk4::EventControllerKey::new();
    let palette_window_clone2 = palette_window.clone();
    controller.connect_key_pressed(move |_, key, _, _| {
        if key == gdk4::Key::Escape {
            palette_window_clone2.close();
            return glib::Propagation::Stop;
        }
        glib::Propagation::Proceed
    });
    palette_window.add_controller(controller);

    // Also activate if Enter is pressed in the search entry.
    let list_box_clone2 = list_box.clone();
    let on_activate_clone2 = on_activate.clone();
    let palette_window_clone3 = palette_window.clone();
    search_entry.connect_activate(move |_| {
        if let Some(row) = list_box_clone2.selected_row() {
            let id = row.widget_name();
            on_activate_clone2(id.as_str());
            palette_window_clone3.close();
        } else {
            // Activate the first visible row.
            let mut child = list_box_clone2.first_child();
            while let Some(row_widget) = child {
                let next = row_widget.next_sibling();
                if row_widget.is_visible() {
                    if let Some(row) = row_widget.downcast_ref::<gtk4::ListBoxRow>() {
                        let id = row.widget_name();
                        on_activate_clone2(id.as_str());
                        palette_window_clone3.close();
                        return;
                    }
                }
                child = next;
            }
        }
    });

    palette_window
}
