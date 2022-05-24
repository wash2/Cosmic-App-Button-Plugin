// SPDX-License-Identifier: MPL-2.0-only

use apps_window::CosmicPanelAppButtonWindow;
use gtk4::gdk::Display;
use gtk4::{
    gio::{self, ApplicationFlags},
    glib,
    prelude::*,
    CssProvider, StyleContext,
};

mod apps_window;
mod localize;
mod utils;

const ID: &str = "com.system76.CosmicPanelAppButton";

pub fn localize() {
    let localizer = crate::localize::localizer();
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();

    if let Err(error) = localizer.select(&requested_languages) {
        eprintln!("Error while loading language for App List {}", error);
    }
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    StyleContext::add_provider_for_display(
        &Display::default().unwrap(),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn main() {
    // Initialize logger
    pretty_env_logger::init();
    glib::set_application_name("Cosmic Panel App Button");

    localize();

    gio::resources_register_include!("compiled.gresource").unwrap();
    let app = gtk4::Application::new(Some(ID), ApplicationFlags::default());

    app.connect_activate(|app| {
        load_css();
        let window = CosmicPanelAppButtonWindow::new(app);

        window.show();
    });
    app.run();
}
