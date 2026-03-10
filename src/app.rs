//! Application bootstrap.
//!
//! `MyTerminalApp` wraps `adw::Application` and wires up the `activate`
//! signal (called once per application instance) to open the main window.

use crate::settings::Settings;
use crate::themes::ThemeManager;
use crate::window::MyTerminalWindow;
use gtk4::prelude::*;
use libadwaita as adw;
use std::cell::RefCell;
use std::rc::Rc;

/// Application ID used by DBus / GApplication.
pub const APP_ID: &str = "com.github.myterminal";

/// Top-level application wrapper.
pub struct MyTerminalApp {
    app: adw::Application,
}

impl MyTerminalApp {
    /// Create and configure a new application instance.
    pub fn new() -> Self {
        let app = adw::Application::builder()
            .application_id(APP_ID)
            .flags(gio::ApplicationFlags::empty())
            .build();

        let app_clone = app.clone();
        app.connect_activate(move |_| {
            Self::on_activate(&app_clone);
        });

        // Register keyboard accelerators.
        Self::register_accels(&app);

        MyTerminalApp { app }
    }

    /// Delegate to GApplication::run so callers get the exit code.
    pub fn run(&self) -> i32 {
        self.app.run_with_args::<String>(&[]).into()
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    fn on_activate(app: &adw::Application) {
        // Load (or create) settings.
        let settings = Rc::new(RefCell::new(
            Settings::load().unwrap_or_else(|e| {
                log::warn!("Failed to load settings, using defaults: {e}");
                Settings::default()
            }),
        ));

        // Build the theme manager from settings.
        let theme_manager = Rc::new(RefCell::new(
            ThemeManager::new(&settings.borrow().theme),
        ));

        // Create and present the main window.
        let window = MyTerminalWindow::new(app, settings, theme_manager);
        window.present();
    }

    /// Register all global keyboard accelerators that map to GActions.
    fn register_accels(app: &adw::Application) {
        let accels: &[(&str, &[&str])] = &[
            ("win.new-tab", &["<Control><Shift>t"]),
            ("win.close-tab", &["<Control><Shift>w"]),
            ("win.next-tab", &["<Control>Tab"]),
            ("win.prev-tab", &["<Control><Shift>Tab"]),
            ("win.search", &["<Control><Shift>f"]),
            ("win.command-palette", &["<Control><Shift>p"]),
            ("win.zoom-in", &["<Control>plus", "<Control>equal"]),
            ("win.zoom-out", &["<Control>minus"]),
            ("win.zoom-reset", &["<Control>0"]),
            ("win.split-horizontal", &["<Control><Shift>d"]),
            ("win.split-vertical", &["<Control>d"]),
            ("win.open-settings", &["<Control>comma"]),
            ("win.copy", &["<Control><Shift>c"]),
            ("win.paste", &["<Control><Shift>v"]),
        ];

        for (action, shortcuts) in accels {
            app.set_accels_for_action(action, shortcuts);
        }
    }
}
