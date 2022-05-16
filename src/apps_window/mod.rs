// SPDX-License-Identifier: MPL-2.0-only

use crate::fl;
use cascade::cascade;
use gtk4::{
    gdk::Display,
    gio::{self, AppLaunchContext, DesktopAppInfo, Icon},
    glib::{self, Object},
    prelude::*,
    Application, Button, IconTheme,
};
use std::process::Command;

mod imp;

const APPLIBRARY: &str = "com.system76.CosmicAppLibrary.desktop";

glib::wrapper! {
    pub struct CosmicDockAppButtonWindow(ObjectSubclass<imp::CosmicDockAppButtonWindow>)
        @extends gtk4::ApplicationWindow, gtk4::Window, gtk4::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk4::Accessible, gtk4::Buildable,
                    gtk4::ConstraintTarget, gtk4::Native, gtk4::Root, gtk4::ShortcutManager;
}

impl CosmicDockAppButtonWindow {
    pub fn new(app: &Application) -> Self {
        let self_: Self = Object::new(&[("application", app)])
            .expect("Failed to create `CosmicDockAppButtonWindow`.");

        cascade! {
            &self_;
            ..set_width_request(1);
            ..set_height_request(1);
            ..set_decorated(false);
            ..set_resizable(false);
            ..set_title(Some(&fl!("cosmic-dock-app-button")));
            ..add_css_class("root_window");
        };

        if let Some(apps_desktop_info) =
            DesktopAppInfo::new(APPLIBRARY)
        {
            let app_button = Button::new();
            app_button.add_css_class("apps");
            let icon = apps_desktop_info
                .icon()
                .unwrap_or(Icon::for_string("image-missing").expect("Failed to set default icon"));

            let image = gtk4::Image::from_gicon(&icon);
            image.set_icon_size(gtk4::IconSize::Large);
            image.set_pixel_size(64);
            app_button.set_child(Some(&image));

            app_button.connect_clicked(move |_| {
                let _ = Command::new("xdg-shell-wrapper").env_remove("WAYLAND_SOCKET").arg(apps_desktop_info.string("Exec").unwrap().as_str()).spawn();
            });
            self_.set_child(Some(&app_button));
        } else {
            panic!("Cosmic applications library is not installed");
        }

        self_
    }
}
