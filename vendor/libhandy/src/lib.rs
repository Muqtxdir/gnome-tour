// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![allow(deprecated)]
#![allow(dead_code)]

extern crate glib_sys;
extern crate gobject_sys;
extern crate gtk_sys;
extern crate libhandy_sys as handy_sys;
extern crate pango;
#[macro_use]
extern crate glib;
extern crate bitflags;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate gdk_pixbuf_sys;
extern crate gdk_sys;
extern crate gio;
extern crate gio_sys;
extern crate gtk;
extern crate lazy_static;
extern crate libc;

/// Asserts that this is the main thread and `gtk::init` has been called.
macro_rules! assert_initialized_main_thread {
    () => {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("Libhandy may only be used from the main thread.");
            } else {
                panic!("Gtk has to be initialized before using libhandy.");
            }
        }
    };
}

macro_rules! skip_assert_initialized {
    () => {};
}

#[macro_use]
pub mod subclass;

pub use glib::Error;
pub mod prelude;
pub use prelude::*;
#[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
#[cfg_attr(feature = "cargo-clippy", allow(useless_transmute))]
mod auto;
#[cfg(any(feature = "v0_0_8", feature = "dox"))]
mod value_object;
pub use auto::functions::init;
pub use auto::*;
#[cfg(any(feature = "v0_0_8", feature = "dox"))]
pub use value_object::*;
