// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Event;
use glib::translate::*;
use std::fmt;
use std::mem;

glib::glib_wrapper! {
    pub struct ConfigureEvent(Object<ffi::GdkConfigureEvent>) @extends Event;

    match fn {
        get_type => || ffi::gdk_configure_event_get_type(),
    }
}

impl ConfigureEvent {
    pub fn get_size(&self) -> (i32, i32) {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            ffi::gdk_configure_event_get_size(
                self.to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            let width = width.assume_init();
            let height = height.assume_init();
            (width, height)
        }
    }
}

impl fmt::Display for ConfigureEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ConfigureEvent")
    }
}