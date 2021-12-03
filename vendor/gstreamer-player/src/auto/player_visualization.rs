// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use gst_player_sys;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PlayerVisualization(Boxed<gst_player_sys::GstPlayerVisualization>);

    match fn {
        copy => |ptr| gst_player_sys::gst_player_visualization_copy(mut_override(ptr)),
        free => |ptr| gst_player_sys::gst_player_visualization_free(ptr),
        get_type => || gst_player_sys::gst_player_visualization_get_type(),
    }
}

unsafe impl Send for PlayerVisualization {}
unsafe impl Sync for PlayerVisualization {}