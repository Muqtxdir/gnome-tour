// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib::translate::*;
use std::fmt;
use ActionGroup;
use ActionMap;

glib_wrapper! {
    pub struct SimpleActionGroup(Object<gio_sys::GSimpleActionGroup, gio_sys::GSimpleActionGroupClass, SimpleActionGroupClass>) @implements ActionGroup, ActionMap;

    match fn {
        get_type => || gio_sys::g_simple_action_group_get_type(),
    }
}

impl SimpleActionGroup {
    pub fn new() -> SimpleActionGroup {
        unsafe { from_glib_full(gio_sys::g_simple_action_group_new()) }
    }
}

impl Default for SimpleActionGroup {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SIMPLE_ACTION_GROUP: Option<&SimpleActionGroup> = None;

impl fmt::Display for SimpleActionGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SimpleActionGroup")
    }
}
