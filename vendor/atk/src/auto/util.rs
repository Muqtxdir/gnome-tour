// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use atk_sys;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct Util(Object<atk_sys::AtkUtil, atk_sys::AtkUtilClass, UtilClass>);

    match fn {
        get_type => || atk_sys::atk_util_get_type(),
    }
}

impl Util {}

pub const NONE_UTIL: Option<&Util> = None;

impl fmt::Display for Util {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Util")
    }
}
