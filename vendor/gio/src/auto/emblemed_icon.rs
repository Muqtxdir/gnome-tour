// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use gobject_sys;
use std::fmt;
use Emblem;
use Icon;

glib_wrapper! {
    pub struct EmblemedIcon(Object<gio_sys::GEmblemedIcon, gio_sys::GEmblemedIconClass, EmblemedIconClass>) @implements Icon;

    match fn {
        get_type => || gio_sys::g_emblemed_icon_get_type(),
    }
}

impl EmblemedIcon {
    pub fn new<P: IsA<Icon>>(icon: &P, emblem: Option<&Emblem>) -> EmblemedIcon {
        unsafe {
            from_glib_full(gio_sys::g_emblemed_icon_new(
                icon.as_ref().to_glib_none().0,
                emblem.to_glib_none().0,
            ))
        }
    }
}

pub const NONE_EMBLEMED_ICON: Option<&EmblemedIcon> = None;

pub trait EmblemedIconExt: 'static {
    fn add_emblem(&self, emblem: &Emblem);

    fn clear_emblems(&self);

    fn get_emblems(&self) -> Vec<Emblem>;

    fn get_icon(&self) -> Option<Icon>;

    fn get_property_gicon(&self) -> Option<Icon>;
}

impl<O: IsA<EmblemedIcon>> EmblemedIconExt for O {
    fn add_emblem(&self, emblem: &Emblem) {
        unsafe {
            gio_sys::g_emblemed_icon_add_emblem(
                self.as_ref().to_glib_none().0,
                emblem.to_glib_none().0,
            );
        }
    }

    fn clear_emblems(&self) {
        unsafe {
            gio_sys::g_emblemed_icon_clear_emblems(self.as_ref().to_glib_none().0);
        }
    }

    fn get_emblems(&self) -> Vec<Emblem> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(gio_sys::g_emblemed_icon_get_emblems(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_icon(&self) -> Option<Icon> {
        unsafe {
            from_glib_none(gio_sys::g_emblemed_icon_get_icon(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_property_gicon(&self) -> Option<Icon> {
        unsafe {
            let mut value = Value::from_type(<Icon as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"gicon\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `gicon` getter")
        }
    }
}

impl fmt::Display for EmblemedIcon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EmblemedIcon")
    }
}