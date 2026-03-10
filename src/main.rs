//! MyTerminal — entry point.
//!
//! Initialises logging, builds the GLib application, and hands control to
//! the GTK main loop.

mod app;
mod commands;
mod settings;
mod tabs;
mod terminal;
mod themes;
mod ui;
mod window;

use app::MyTerminalApp;

fn main() {
    // Initialise the env_logger; use RUST_LOG=debug / info / warn / error.
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    log::info!("Starting MyTerminal {}", env!("CARGO_PKG_VERSION"));

    let app = MyTerminalApp::new();
    std::process::exit(app.run());
}
