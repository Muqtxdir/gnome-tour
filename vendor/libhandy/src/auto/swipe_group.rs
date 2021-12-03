// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use gtk;
use handy_sys;
use std::fmt;
use Swipeable;

glib_wrapper! {
    pub struct SwipeGroup(Object<handy_sys::HdySwipeGroup, handy_sys::HdySwipeGroupClass, SwipeGroupClass>) @implements gtk::Buildable;

    match fn {
        get_type => || handy_sys::hdy_swipe_group_get_type(),
    }
}

impl SwipeGroup {
    pub fn new() -> SwipeGroup {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(handy_sys::hdy_swipe_group_new()) }
    }
}

impl Default for SwipeGroup {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SWIPE_GROUP: Option<&SwipeGroup> = None;

pub trait SwipeGroupExt: 'static {
    fn add_swipeable<P: IsA<Swipeable>>(&self, swipeable: &P);

    fn get_swipeables(&self) -> Vec<Swipeable>;

    fn remove_swipeable<P: IsA<Swipeable>>(&self, swipeable: &P);
}

impl<O: IsA<SwipeGroup>> SwipeGroupExt for O {
    fn add_swipeable<P: IsA<Swipeable>>(&self, swipeable: &P) {
        unsafe {
            handy_sys::hdy_swipe_group_add_swipeable(
                self.as_ref().to_glib_none().0,
                swipeable.as_ref().to_glib_none().0,
            );
        }
    }

    fn get_swipeables(&self) -> Vec<Swipeable> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(handy_sys::hdy_swipe_group_get_swipeables(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove_swipeable<P: IsA<Swipeable>>(&self, swipeable: &P) {
        unsafe {
            handy_sys::hdy_swipe_group_remove_swipeable(
                self.as_ref().to_glib_none().0,
                swipeable.as_ref().to_glib_none().0,
            );
        }
    }
}

impl fmt::Display for SwipeGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SwipeGroup")
    }
}