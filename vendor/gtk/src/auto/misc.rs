// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use gtk_sys;
use std::fmt;
use Buildable;
use Widget;

glib_wrapper! {
    pub struct Misc(Object<gtk_sys::GtkMisc, gtk_sys::GtkMiscClass, MiscClass>) @extends Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_misc_get_type(),
    }
}

impl Misc {}

pub const NONE_MISC: Option<&Misc> = None;

impl fmt::Display for Misc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Misc")
    }
}
