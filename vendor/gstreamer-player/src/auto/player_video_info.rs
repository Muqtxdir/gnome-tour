// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use gst_player_sys;
use PlayerStreamInfo;

glib_wrapper! {
    pub struct PlayerVideoInfo(Object<gst_player_sys::GstPlayerVideoInfo, gst_player_sys::GstPlayerVideoInfoClass, PlayerVideoInfoClass>) @extends PlayerStreamInfo;

    match fn {
        get_type => || gst_player_sys::gst_player_video_info_get_type(),
    }
}

impl PlayerVideoInfo {
    pub fn get_bitrate(&self) -> i32 {
        unsafe { gst_player_sys::gst_player_video_info_get_bitrate(self.to_glib_none().0) }
    }

    pub fn get_height(&self) -> i32 {
        unsafe { gst_player_sys::gst_player_video_info_get_height(self.to_glib_none().0) }
    }

    pub fn get_max_bitrate(&self) -> i32 {
        unsafe { gst_player_sys::gst_player_video_info_get_max_bitrate(self.to_glib_none().0) }
    }

    pub fn get_width(&self) -> i32 {
        unsafe { gst_player_sys::gst_player_video_info_get_width(self.to_glib_none().0) }
    }
}

unsafe impl Send for PlayerVideoInfo {}
unsafe impl Sync for PlayerVideoInfo {}