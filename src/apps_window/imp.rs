// SPDX-License-Identifier: MPL-2.0-only

use gtk4::{glib, subclass::prelude::*};
use once_cell::sync::OnceCell;
// Object holding the state
#[derive(Default)]

pub struct CosmicDockAppButtonWindow {}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CosmicDockAppButtonWindow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "CosmicDockAppButtonWindow";
    type Type = super::CosmicDockAppButtonWindow;
    type ParentType = gtk4::ApplicationWindow;
}

// Trait shared by all GObjects
impl ObjectImpl for CosmicDockAppButtonWindow {}

// Trait shared by all widgets
impl WidgetImpl for CosmicDockAppButtonWindow {}

// Trait shared by all windows
impl WindowImpl for CosmicDockAppButtonWindow {}

// Trait shared by all application
impl ApplicationWindowImpl for CosmicDockAppButtonWindow {}
